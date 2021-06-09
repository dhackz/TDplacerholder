use crate::BLOCK_SIZE;

use ggez::{
    graphics,
    GameResult,
    Context,
};

pub const BASE_SIZE: f32 = 60.0;
pub const BASE_PADDING: f32 = 5.0;

pub struct Base {
    pub pos: (f32, f32),
}

impl Base {
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, BASE_SIZE, BASE_SIZE].into(),
            ggez::graphics::Color::new(0.0, 1.0, 0.0, 1.0),
        )?;

        let location = (ggez::mint::Point2 {
            x: self.pos.0 * BLOCK_SIZE + BASE_PADDING,
            y: self.pos.1 * BLOCK_SIZE + BASE_SIZE / 2.0 - BASE_PADDING,
        },);

        graphics::draw(ctx, &rectangle, location)?;
        Ok(())
    }
}
