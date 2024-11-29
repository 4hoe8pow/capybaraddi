use crate::actor_units::player::{entry, Action};
use crate::actor_units::{court::Court, player::Player};
use std::collections::HashMap;

pub struct RaidEnv {
    pub(crate) raider: Player,
    pub(crate) anti_list: HashMap<i32, Player>,
    pub(crate) court: Court,
}

pub trait SimulationEnvironment {
    // 現在の状態を取得
    fn current_state(&self) -> (f32, f32);

    // 行動を実行して次の状態と報酬を取得
    fn step(&mut self, action: Action) -> (Player, f32);

    // 次の状態を計算するロジックを提供
    fn calculate_next_state(&self, action: Action) -> (f32, f32);

    // 報酬を計算するロジックを提供
    fn calculate_reward(&self) -> f32;
}

impl RaidEnv {
    pub fn new(num_anti: usize) -> Result<Self, &'static str> {
        let (raider, anti_list) = entry(num_anti)?;
        Ok(Self {
            raider,
            anti_list,
            court: Court::default(),
        })
    }

    /// レイダーとすべてのアンティとの距離を計算する関数
    pub fn distances_to_all_antis(&self) -> Vec<f32> {
        self.anti_list
            .values()
            .map(|anti| {
                ((self.raider.position.0 - anti.position.0).powi(2)
                    + (self.raider.position.1 - anti.position.1).powi(2))
                .sqrt()
            })
            .collect()
    }
}

impl SimulationEnvironment for RaidEnv {
    /// 現在の状態（レイダーの位置）を取得
    fn current_state(&self) -> (f32, f32) {
        self.raider.position
    }

    /// 行動を実行し、次の状態と報酬を返す
    fn step(&mut self, action: Action) -> (Player, f32) {
        // 次の状態を計算
        let next_position = self.calculate_next_state(action);
        self.raider.position = next_position;

        // 境界外かチェック
        if self.court.is_out_of_bounds(self.raider) {
            return (self.raider, -10.0); // ペナルティ
        }

        // 報酬を計算
        let reward = self.calculate_reward();

        // レイド成功条件を確認
        if self.court.is_raid_successful(&self.raider) {
            return (self.raider, reward + 50.0); // 成功ボーナス
        }

        (self.raider, reward)
    }

    /// 次の状態を計算
    fn calculate_next_state(&self, action: Action) -> (f32, f32) {
        let (x, y) = self.raider.position;
        match action {
            Action::MoveForward => (x + 1.0, y),
            Action::MoveBackward => (x - 1.0, y),
            Action::MoveLeft => (x, y - 1.0),
            Action::MoveRight => (x, y + 1.0),
            Action::Retreat => (x - 2.0, y), // 素早く戻る例
            _ => (x, y),                     // その他のアクションは位置を変更しない
        }
    }

    /// 報酬を計算
    fn calculate_reward(&self) -> f32 {
        // アンティとの距離でのペナルティを合計
        let penalty_for_closeness: f32 = self
            .distances_to_all_antis()
            .iter()
            .filter(|&&distance| distance < 1.0)
            .count() as f32
            * -5.0;

        // 境界付近でのペナルティ
        let boundary_penalty = if self.raider.is_near_boundary() {
            -2.0
        } else {
            0.0
        };

        // 総報酬を計算
        penalty_for_closeness + boundary_penalty
    }
}
