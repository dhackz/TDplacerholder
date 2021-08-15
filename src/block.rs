use ggez::{graphics, Context, GameResult};

use crate::utils::Scale;

pub const BLOCK_SIZE: f32 = 35.0;

pub struct Block {
    pub position: [f32; 2],
}

impl Block {
    pub fn draw(&mut self, ctx: &mut Context, scale: Scale) -> GameResult {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            scale.to_viewport_rect([0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE].into()),
            ggez::graphics::Color::new(0.1, 0.4, 0.0, 1.0),
        )?;

        let location = (
            scale.to_viewport_point(self.position[0] * BLOCK_SIZE, self.position[1] * BLOCK_SIZE),
        );

        graphics::draw(ctx, &rectangle, location)?;
        Ok(())
    }
}
