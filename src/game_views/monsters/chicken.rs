use crate::{
    animation_system::Animation,
    asset_system::AssetManager,
    game_components::monsters::{Chicken, Monster},
    game_views::monsters::MonsterView,
    utils::Direction,
};

use ggez::{
    mint::Point2,
    Context, GameResult,
};

/// Responsible for drawing the chicken to the screen and managing view related
/// attributes, such as animations, of the chicken game component.
pub struct ChickenView {
    pub chicken: Chicken,
    pub animations: Animation, // note: only have a single animation for now.
}

impl ChickenView {
    pub fn new(asset_manager: &AssetManager) -> ChickenView {
        // The idea is that mosnter, their properties, and their animations
        // will be defined in ron files later so they can be changed without
        // having to recompile the game every single time you want to change an
        // animation/monster property etc. For now we do it this way since it
        // is simpler to get started.
        ChickenView {
            chicken: Chicken::new(),
            animations: Animation {
                current_sprite: 0,
                next_sprite_interval: 500,
                next_sprite_time: 0,
                sprites: vec![
                    asset_manager.monster_assets.chicken_assets.walking_sprites[0].clone(),
                    asset_manager.monster_assets.chicken_assets.walking_sprites[1].clone(),
                ],
            },
        }
    }
}

impl MonsterView for ChickenView {
    fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult {
        let chicken_sprite = &asset_manager.monster_assets.chicken_assets.walking_sprites[0];
        let half_width = chicken_sprite.width() as f32 / 2.0;
        let half_height = chicken_sprite.height() as f32 / 2.0;

        if self.chicken.direction == Direction::Left {
            // Flipping along y-axis causes image to end up at a position
            // (-width, 0). Offsetting with (+width/2, -height/2) makes the
            // image center end up at (0,0).
            let offset_position = Point2 {
                x: self.chicken.position[0] + half_width,
                y: self.chicken.position[1] - half_height,
            };

            // Flip along y-axis. Scale then move.
            self.animations.draw(ctx, self.chicken.direction, asset_manager, offset_position)?;
        } else {
            let offset_position = Point2 {
                x: self.chicken.position[0] - half_width + 10.0, /* Image specific x-offset */
                y: self.chicken.position[1] - half_height,
            };
            self.animations.draw(ctx, self.chicken.direction, asset_manager, offset_position)?;
        }

        Ok(())
    }

    fn get_monster_mut(&mut self) -> &mut dyn Monster {
        &mut self.chicken
    }

    fn get_monster(&self) -> &dyn Monster {
        &self.chicken
    }
}
