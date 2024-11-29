use crate::actor_units::player::entry;
use crate::actor_units::{court::Court, player::Player};
use crate::agent::Action;
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
}

impl SimulationEnvironment for RaidEnv {
    fn current_state(&self) -> (f32, f32) {
        self.raider.position // レイダーの位置を返す
    }

    fn step(&mut self, action: Action) -> (Player, f32) {
        // 行動による状態遷移処理を行う
        match action {
            Action::MoveForward => self.raider.position.0 += 1.0,
            Action::MoveBackward => self.raider.position.0 -= 1.0,
            Action::MoveLeft => self.raider.position.1 -= 1.0,
            Action::MoveRight => self.raider.position.1 += 1.0,
            _ => {}
        }

        let reward = self.calculate_reward();

        (self.raider, reward)
    }

    fn calculate_next_state(&self, action: Action) -> (f32, f32) {
        let (x, y) = self.raider.position;
        match action {
            Action::MoveForward => (x + 1.0, y),
            Action::MoveBackward => (x - 1.0, y),
            Action::MoveLeft => (x, y - 1.0),
            Action::MoveRight => (x, y + 1.0),
            _ => (x, y),
        }
    }

    fn calculate_reward(&self) -> f32 {
        1.0
    }
}
