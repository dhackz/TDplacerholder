use ggez::{graphics, Context, GameResult};

pub const BLOCK_SIZE: f32 = 35.0;

pub struct Block {
    pub pos: (f32, f32),
}

impl Block {
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE].into(),
            ggez::graphics::Color::new(0.1, 0.4, 0.0, 1.0),
        )?;

        let location = (ggez::mint::Point2 {
            x: self.pos.0 * BLOCK_SIZE,
            y: self.pos.1 * BLOCK_SIZE,
        },);

        graphics::draw(ctx, &rectangle, location)?;
        Ok(())
    }
}
