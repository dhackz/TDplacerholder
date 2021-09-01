use crate::asset_system::AssetManager;
use crate::utils::Direction;

use ggez::{
    timer,
    graphics::{self, DrawParam, Image},
    mint::Point2,
    Context, GameResult,
};

pub struct Animation {
    pub current_sprite: usize,
    pub next_sprite_interval: u32,
    pub next_sprite_time: u128,
    pub sprites: Vec<Image>,
}

/// An Animation manages the timing and drawing of a single looping animation.
impl Animation {
    pub fn draw(
        &mut self,
        ctx: &mut Context,
        direction: Direction,
        asset_manager: &AssetManager,
        position: Point2<f32>,
    ) -> GameResult {
        let duration = timer::time_since_start(ctx);
        let current_time = duration.as_millis();

        if current_time > self.next_sprite_time {
            self.next_sprite_time = current_time + self.next_sprite_interval as u128;
            self.current_sprite = (self.current_sprite + 1) % self.sprites.len();
        }

        let scale = match direction {
            Direction::Left => [-1.0, 1.0],
            Direction::Right => [1.0, 1.0],
        };

        graphics::draw(
            ctx,
            &asset_manager.monster_assets.chicken_assets.walking_sprites[self.current_sprite],
            DrawParam::default()
                        .scale(scale)
                        .dest(position),
        )?;

        Ok(())
    }
}
