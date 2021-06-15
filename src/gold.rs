use crate::asset_manager::AssetManager;

use log::debug;

use ggez::{graphics, Context, GameResult};

pub struct GoldPile {
    pub position: [f32; 2],
    pub value: u32,
}

impl GoldPile {
    pub fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult {
        let location = (ggez::mint::Point2 {
            x: self.position[0],
            y: self.position[1] - 10.0,
        },);

        debug!("GoldPile: draw: drawing at location ({:?})", location);
        graphics::draw(ctx, &asset_manager.gold_sprite, location)?;

        Ok(())
    }
}
