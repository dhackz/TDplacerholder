use ggez::{Context, GameResult, graphics};
use ggez::mint::Point2;


pub const BLOCK_SIZE: f32 = 35.0;

pub struct Block {
    pub position: [f32; 2],
}

impl Block {
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE].into(),
            ggez::graphics::Color::new(0.1, 0.4, 0.0, 1.0),
        )?;

        let location = (Point2 {
            x: self.position[0] * BLOCK_SIZE,
            y: self.position[1] * BLOCK_SIZE,

        },);

        graphics::draw(ctx, &rectangle, location)?;
        Ok(())
    }
}
