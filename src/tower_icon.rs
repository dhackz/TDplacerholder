use crate::asset_manager::AssetManager;
use crate::towers::tower::TowerType;
use crate::utils::Scale;

use ggez::graphics::{self, DrawParam};
use ggez::mint::Point2;
use ggez::Context;
use ggez::GameResult;

pub const TOWER_ICON_SIZE: f32 = 50.0;

pub struct TowerIcon {
    pub tower_type: TowerType,
}

impl TowerIcon {
    pub fn draw(
        &self,
        ctx: &mut Context,
        scale: Scale,
        asset_manager: &AssetManager,
        location: Point2<f32>,
        selected: bool
    ) -> GameResult {
        let asset = match self.tower_type {
            TowerType::Basic => {
                if selected {
                    &asset_manager.builder_ui_assets.tower_selected_sprite
                } else {
                    &asset_manager.builder_ui_assets.tower_sprite
                }
            },
            TowerType::Ninja => {
                if selected {
                    &asset_manager.builder_ui_assets.ninja_tower_selected_sprite
                } else {
                    &asset_manager.builder_ui_assets.ninja_tower_sprite
                }
            },
        };

        // Destination isn't scaled by DrawParam.
        let destination = scale.to_viewport_point(location.x, location.y);
        debug!("draw: location: {:?}", destination);
        graphics::draw(
            ctx,
            asset,
            DrawParam::default()
                .scale([scale.x, scale.y])
                .dest(destination),
        )?;

        Ok(())
    }
}
