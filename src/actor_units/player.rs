use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Default, Clone, Copy)]
pub struct Player {
    pub id: i32,
    pub height: f32,          // メートル
    pub weight: f32,          // キログラム
    pub position: (f32, f32), // 位置を座標で保持
    pub is_struggling: bool,  // ストラグル中かどうか
}

impl Player {
    pub fn new(
        id: i32,
        height: f32,
        weight: f32,
        position: (f32, f32),
        is_struggling: bool,
    ) -> Self {
        Self {
            id,
            height,
            weight,
            position,
            is_struggling,
        }
    }

    /// プレイヤーがストラグル可能かどうかを判定
    pub fn can_struggle(&self) -> bool {
        // 例: 現在ストラグル中でない場合にのみストラグル可能
        !self.is_struggling
    }

    /// プレイヤーがコートの境界付近にいるかどうかを判定
    pub fn is_near_boundary(&self) -> bool {
        // 境界を (0, 0) ～ (20, 10) と仮定し、境界付近(距離1.0未満)か判定
        let (x, y) = self.position;
        x <= 1.0 || x >= 19.0 || y <= 1.0 || y >= 9.0
    }

    /// プレイヤーが相手を捕獲可能かどうかを判定
    pub fn can_capture(&self) -> bool {
        // 例: 捕獲可能な条件 (詳細は具体的なルールに基づいて調整)
        self.height >= 1.7 && self.weight >= 60.0
    }
}

// PartialEqを実装
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.position == other.position
    }
}

// Eqを実装
impl Eq for Player {}

// Hashを実装
impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.position.0.to_bits().hash(state); // f32のビット表現をハッシュ化
        self.position.1.to_bits().hash(state); // f32のビット表現をハッシュ化
    }
}

// エントリ生成用のファクトリ関数
pub fn entry(num_anti: usize) -> Result<(Player, HashMap<i32, Player>), &'static str> {
    if num_anti == 0 || num_anti > 8 {
        return Err("アンティの人数は1から7の間で指定してください。");
    }

    let raider = Player::new(0, 1.75, 70.0, (0.0, 0.0), false);
    let mut anti_list = HashMap::new();
    for i in 1..=num_anti as i32 {
        anti_list.insert(
            i,
            Player::new(
                i,
                1.8 - (i as f32 * 0.1),
                75.0 - (i as f32 * 2.0),
                (10.0 + (i as f32 * 2.0), 5.0 + (i as f32 * 1.0)),
                false,
            ),
        );
    }
    Ok((raider, anti_list))
}
