use super::player::Player;

#[derive(Debug)]
pub struct Court {
    pub width_x: f32,  // コートのx軸長
    pub height_y: f32, // コートのy軸長
    pub width_z: f32,  // コートのz軸幅
}

impl Court {
    pub fn enable_lobby(&mut self) {
        self.width_x = 10.0;
    }

    pub fn disable_lobby(&mut self) {
        self.width_x = 8.0;
    }

    pub fn is_out_of_bounds(&self, player: Player) -> bool {
        let (x, y) = player.position;
        x < 0.0 || x >= self.width_x || y > self.width_z
    }

    pub fn is_raid_successful(&self, player: &Player) -> bool {
        // レイダーがコートの左側に到達した時、ストラグル中なら成功
        player.position.0 < 0.0 && player.is_struggling
    }
}

impl Default for Court {
    fn default() -> Self {
        Self {
            width_x: 8.0,
            height_y: 10.0,
            width_z: 6.5,
        }
    }
}
