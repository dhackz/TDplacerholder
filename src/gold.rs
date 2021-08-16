use crate::asset_manager::AssetManager;
use crate::utils::Scale;

use log::debug;

use ggez::graphics::DrawParam;
use ggez::{graphics, Context, GameResult};

pub struct GoldPile {
    pub position: [f32; 2],
    pub value: u32,
}

impl GoldPile {
    pub fn draw(
        &mut self,
        ctx: &mut Context,
        scale: Scale,
        asset_manager: &AssetManager,
    ) -> GameResult {
        let location = scale.to_viewport_point(self.position[0], self.position[1] - 10.0);

        debug!("GoldPile: draw: drawing at location ({:?})", location);
        graphics::draw(
            ctx,
            &asset_manager.item_assets.gold_sprite,
            DrawParam::default()
                .scale([scale.x, scale.y])
                .dest(location),
        )?;

        Ok(())
    }
}
