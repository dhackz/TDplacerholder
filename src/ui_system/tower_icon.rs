use crate::{asset_system::AssetManager, game_components::towers::TowerType};

use ggez::{
    graphics::{self, DrawParam},
    mint::Point2,
    Context, GameResult,
};

pub const TOWER_ICON_SIZE: f32 = 50.0;

pub struct TowerIcon {
    pub tower_type: TowerType,
}

impl TowerIcon {
    pub fn draw(
        &self,
        ctx: &mut Context,
        asset_manager: &AssetManager,
        location: Point2<f32>,
        selected: bool,
    ) -> GameResult {
        let asset = match self.tower_type {
            TowerType::Basic => {
                if selected {
                    &asset_manager.builder_ui_assets.tower_selected_sprite
                } else {
                    &asset_manager.builder_ui_assets.tower_sprite
                }
            }
            TowerType::Ninja => {
                if selected {
                    &asset_manager.builder_ui_assets.ninja_tower_selected_sprite
                } else {
                    &asset_manager.builder_ui_assets.ninja_tower_sprite
                }
            }
        };

        // Destination isn't scaled by DrawParam.
        debug!("draw: location: {:?}", location);
        graphics::draw(ctx, asset, DrawParam::default().dest(location))?;

        Ok(())
    }
}
