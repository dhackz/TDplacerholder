use crate::{asset_system::AssetManager, game_components::BLOCK_SIZE};

use ggez::{
    graphics::{self, DrawParam},
    mint::Point2,
    Context, GameResult,
};

pub const BASE_SIZE: f32 = 60.0;
pub const BASE_PADDING: f32 = 5.0;

pub struct Base {
    pub position: [f32; 2],
}

impl Base {
    pub fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult {
        let location = Point2 {
            x: self.position[0] * BLOCK_SIZE + BASE_PADDING,
            y: self.position[1] * BLOCK_SIZE + BASE_PADDING,
        };

        graphics::draw(
            ctx,
            &asset_manager.base_assets.base_sprite,
            DrawParam::default().dest(location),
        )?;

        Ok(())
    }

    pub fn is_position_in_base(&self, position: [f32; 2]) -> bool {
        position[0] >= self.position[0] * BLOCK_SIZE - BASE_PADDING
            && position[0] <= self.position[0] * BLOCK_SIZE + BASE_SIZE + 2.0 * BASE_PADDING
            && position[1] >= self.position[1] * BLOCK_SIZE
            && position[1] <= self.position[1] * BLOCK_SIZE + BASE_SIZE + 2.0 * BASE_PADDING
    }
}
