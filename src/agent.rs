use crate::actor_units::player::Player;
use rand::prelude::SliceRandom;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Agent {
    pub(crate) q_table: HashMap<(Player, Action), f32>, // Qテーブル
    pub(crate) learning_rate: f32,
    pub(crate) discount_factor: f32,
    pub(crate) exploration_rate: f32,
    pub(crate) exploration_decay: f32,
}

impl Agent {
    pub fn new(
        learning_rate: f32,
        discount_factor: f32,
        exploration_rate: f32,
        exploration_decay: f32,
    ) -> Self {
        Self {
            q_table: HashMap::new(),
            learning_rate,
            discount_factor,
            exploration_rate,
            exploration_decay,
        }
    }

    // 動的に次の行動候補を生成するメソッド
    pub fn get_next_actions(&self, player: &Player) -> Vec<Action> {
        let mut actions = vec![
            Action::MoveForward,
            Action::MoveBackward,
            Action::MoveLeft,
            Action::MoveRight,
        ];

        // 例えば、条件によって特定のアクションを追加する例
        if player.can_struggle() {
            actions.push(Action::Struggle);
        }
        if player.is_near_boundary() {
            actions.push(Action::Retreat);
        }
        if player.can_capture() {
            actions.push(Action::Capture);
        }

        actions
    }

    // 行動選択器
    pub fn choose_action(&mut self, player: Player, next_actions: &[Action]) -> Action {
        if rand::random::<f32>() < self.exploration_rate {
            // 探索: ランダムに選択
            *next_actions.choose(&mut rand::thread_rng()).unwrap()
        } else {
            // 利用: Q値が最大のアクションを選択
            next_actions
                .iter()
                .map(|&action| (action, *self.q_table.get(&(player, action)).unwrap_or(&0.0)))
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap()
                .0
        }
    }

    // 学習器: Qテーブルを更新
    pub fn update_q_table(
        &mut self,
        player: Player,
        action: Action,
        reward: f32,
        next_player: Player,
        next_actions: &[Action],
    ) {
        let next_max_q = next_actions
            .iter()
            .map(|&next_action| {
                *self
                    .q_table
                    .get(&(next_player, next_action))
                    .unwrap_or(&0.0)
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);

        let q_value = self.q_table.entry((player, action)).or_insert(0.0);
        *q_value += self.learning_rate * (reward + self.discount_factor * next_max_q - *q_value);
    }

    // 探索率の減衰
    pub fn decay_exploration(&mut self) {
        self.exploration_rate *= self.exploration_decay;
    }
}

impl From<Vec<String>> for Agent {
    fn from(args: Vec<String>) -> Self {
        match args.len() {
            4 => {
                let learning_rate = args[0].parse::<f32>().unwrap_or(0.1);
                let discount_factor = args[1].parse::<f32>().unwrap_or(0.9);
                let exploration_rate = args[2].parse::<f32>().unwrap_or(1.0);
                let exploration_decay = args[3].parse::<f32>().unwrap_or(0.995);
                Agent::new(
                    learning_rate,
                    discount_factor,
                    exploration_rate,
                    exploration_decay,
                )
            }
            _ => Agent::default(), // 引数が不足している場合はデフォルトを使用
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Struggle,
    Retreat,
    Avoid,
    Capture,
    PushOut,
}
