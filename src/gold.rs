use crate::asset_manager::AssetManager;

use ggez::{graphics, Context, GameResult};

pub struct GoldPile {
    pub position: [f32; 2],
    pub value: i32,
}

impl GoldPile {
    pub fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult {
        let location = (ggez::mint::Point2 {
            x: self.position[0],
            y: self.position[1] - 10.0,
        },);

        graphics::draw(ctx, &asset_manager.gold_sprite, location)?;

        Ok(())
    }
}
