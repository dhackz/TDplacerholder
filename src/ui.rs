use crate::asset_manager::AssetManager;
use crate::gold::GoldPile;
use crate::utils::Scale;
use crate::{Player, BLOCK_SIZE};

use ggez::{audio::SoundSource, graphics, Context, GameResult};

pub const UI_HEIGHT: f32 = 180.0;

const GOLD_X: f32 = 30.0;
const GOLD_Y: f32 = 30.0;

const HP_X: f32 = 30.0;
const HP_Y: f32 = 50.0;

pub struct TowerIcon {}

#[derive(Debug, Eq, PartialEq)]
pub enum TowerType {
    Basic,
    Ninja,
}

pub struct UI {
    pub build_bar: Vec<TowerIcon>,
    pub selected_tile_rect: Option<[f32; 2]>,
    pub selected_tile_type: TowerType,
}

impl UI {
    pub fn draw(&mut self, ctx: &mut Context, scale: Scale, player: &Player) -> GameResult {
        self.draw_background(ctx, scale)?;
        self.draw_gold(ctx, scale, player)?;
        self.draw_hp(ctx, scale, player)?;
        // self.draw_tower_icons(ctx)?;
        self.draw_selected_tile(ctx, scale)?;
        Ok(())
    }

    fn draw_background(&mut self, ctx: &mut Context, scale: Scale) -> GameResult {
        let screen_size = ggez::graphics::size(ctx);

        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, screen_size.0, UI_HEIGHT * scale.y].into(),
            ggez::graphics::Color::new(0.2, 0.3, 0.4, 1.0),
        )?;

        let location = (scale.to_viewport_point(0.0, screen_size.1 - UI_HEIGHT),);

        graphics::draw(ctx, &rectangle, location)?;
        Ok(())
    }

    fn draw_gold(&mut self, ctx: &mut Context, scale: Scale, player: &Player) -> GameResult {
        let screen_rect = ggez::graphics::screen_coordinates(ctx);

        let text = graphics::Text::new(format!("GOLD: {}", player.gold));
        let location_x = GOLD_X;
        let location_y = screen_rect.h - UI_HEIGHT + GOLD_Y;
        let location = (scale.to_viewport_point(location_x, location_y),);
        graphics::draw(ctx, &text, location)?;
        Ok(())
    }

    fn draw_hp(&mut self, ctx: &mut Context, scale: Scale, player: &Player) -> GameResult {
        let screen_rect = ggez::graphics::screen_coordinates(ctx);

        let text = graphics::Text::new(format!("HP: {}", player.health));
        let location_x = HP_X;
        let location_y = screen_rect.h - UI_HEIGHT + HP_Y;
        let location = (scale.to_viewport_point(location_x, location_y),);
        graphics::draw(ctx, &text, location)?;
        Ok(())
    }

    fn draw_selected_tile(&mut self, ctx: &mut Context, scale: Scale) -> GameResult {
        if let Some(tile) = self.selected_tile_rect {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(3.0),
                scale.to_viewport_rect([0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE].into()),
                ggez::graphics::Color::new(0.5, 0.0, 0.0, 1.0),
            )?;

            let location = (ggez::mint::Point2 {
                x: tile[0],
                y: tile[1],
            },);
            graphics::draw(ctx, &rectangle, location)?;
        }
        Ok(())
    }

    pub fn mouse_motion_event(
        &mut self,
        ctx: &Context,
        scale: Scale,
        x: f32,
        y: f32,
        gold_piles: &mut Vec<GoldPile>,
        player: &mut Player,
        asset_manager: &mut AssetManager,
    ) {
        let screen_rect = ggez::graphics::screen_coordinates(ctx);

        // Check inside game window.
        if x > 0.0 && x < screen_rect.w && y > 0.0 && y < screen_rect.h - UI_HEIGHT * scale.y {
            // Convert x/y to in-game coordinates.
            let scaled_block_size = scale.to_viewport_point(BLOCK_SIZE, BLOCK_SIZE);

            // Change selected_tile.
            self.selected_tile_rect = Some([
                (x / scaled_block_size.x).floor() * scaled_block_size.x,
                (y / scaled_block_size.y).floor() * scaled_block_size.y,
            ]);

            // Check for any gold to pick up.
            gold_piles.retain(|gold_pile| {
                let scaled_xy = scale.to_game_point(x, y);

                let xd = scaled_xy.x - (gold_pile.position[0] + 35.0 / 2.0);
                let yd = scaled_xy.y - (gold_pile.position[1] + 35.0 / 2.0);

                // Within 20px radius.
                if xd * xd + yd * yd < 20.0 * 20.0 {
                    player.gold += gold_pile.value;
                    asset_manager.gold_sound.play().unwrap();
                    false
                } else {
                    true
                }
            });
        } else {
            self.selected_tile_rect = None;
        }
    }
}
