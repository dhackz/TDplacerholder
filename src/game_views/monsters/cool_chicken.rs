use crate::asset_system::AssetManager;
use crate::game_components::monsters::{CoolChicken, Monster};
use crate::game_views::monsters::MonsterView;
use crate::utils::Direction;

use ggez::{
    graphics::{self, DrawParam},
    Context, GameResult,
};

pub struct CoolChickenView {
    pub cool_chicken: CoolChicken,
}

impl MonsterView for CoolChickenView {
    fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult {
        let half_width = asset_manager.monster_assets.cool_chicken_sprite.width() as f32 / 2.0;
        let half_height = asset_manager.monster_assets.cool_chicken_sprite.height() as f32 / 2.0;

        if self.cool_chicken.direction == Direction::Left {
            // Flipping along y-axis causes image to end up at a position
            // (-width, 0). Offsetting with (+width/2, -height/2) makes the
            // image center end up at (0,0).
            let offset_position = [
                self.cool_chicken.position[0] + half_width,
                self.cool_chicken.position[1] - half_height,
            ];

            // Flip along y-axis. Scale then move.
            graphics::draw(
                ctx,
                &asset_manager.monster_assets.cool_chicken_sprite,
                DrawParam::default()
                    .scale([-1.0, 1.0])
                    .dest(offset_position),
            )?;
        } else {
            let offset_position = [
                self.cool_chicken.position[0] - half_width + 10.0, /* Image specific x-offset */
                self.cool_chicken.position[1] - half_height,
            ];
            graphics::draw(
                ctx,
                &asset_manager.monster_assets.cool_chicken_sprite,
                DrawParam::default().dest(offset_position),
            )?;
        }

        Ok(())
    }

    fn get_monster_mut(&mut self) -> &mut dyn Monster {
        &mut self.cool_chicken
    }

    fn get_monster(&self) -> &dyn Monster {
        &self.cool_chicken
    }
}
