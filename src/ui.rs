use crate::asset_manager::AssetManager;
use crate::gold::GoldPile;
use crate::{Player, BLOCK_SIZE};

use ggez::{audio::SoundSource, graphics, Context, GameResult};

pub const WINDOW_HEIGHT: f32 = 600.0;
pub const WINDOW_WIDTH: f32 = 800.0;
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
    pub selected_tile_location: Option<[f32; 2]>,
    pub selected_tile_type: TowerType,
}

impl UI {
    pub fn draw(&mut self, ctx: &mut Context, player: &Player) -> GameResult {
        self.draw_background(ctx)?;
        self.draw_gold(ctx, player)?;
        self.draw_hp(ctx, player)?;
        // self.draw_tower_icons(ctx)?;
        self.draw_selected_tile(ctx)?;
        Ok(())
    }

    fn draw_background(&mut self, ctx: &mut Context) -> GameResult {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, WINDOW_WIDTH, UI_HEIGHT].into(),
            ggez::graphics::Color::new(0.2, 0.3, 0.4, 1.0),
        )?;

        let location = (ggez::mint::Point2 {
            x: 0.0,
            y: WINDOW_HEIGHT - UI_HEIGHT,
        },);
        graphics::draw(ctx, &rectangle, location)?;
        Ok(())
    }

    fn draw_gold(&mut self, ctx: &mut Context, player: &Player) -> GameResult {
        let text = graphics::Text::new(format!("GOLD: {}", player.gold));
        let location_x = GOLD_X;
        let location_y = WINDOW_HEIGHT - UI_HEIGHT + GOLD_Y;
        let location = (ggez::mint::Point2 {
            x: location_x,
            y: location_y,
        },);
        graphics::draw(ctx, &text, location)?;
        Ok(())
    }

    fn draw_hp(&mut self, ctx: &mut Context, player: &Player) -> GameResult {
        let text = graphics::Text::new(format!("HP: {}", player.health));
        let location_x = HP_X;
        let location_y = WINDOW_HEIGHT - UI_HEIGHT + HP_Y;
        let location = (ggez::mint::Point2 {
            x: location_x,
            y: location_y,
        },);
        graphics::draw(ctx, &text, location)?;
        Ok(())
    }

    fn draw_selected_tile(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(tile) = self.selected_tile_location {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(3.0),
                [0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE].into(),
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
        x: f32,
        y: f32,
        gold_piles: &mut Vec<GoldPile>,
        player: &mut Player,
        asset_manager: &mut AssetManager,
    ) {
        // Check inside game window.
        if x > 0.0 && x < WINDOW_WIDTH && y > 0.0 && y < WINDOW_HEIGHT - UI_HEIGHT {
            // Change selected_tile.
            self.selected_tile_location = Some([
                (x / BLOCK_SIZE).floor() * BLOCK_SIZE,
                (y / BLOCK_SIZE).floor() * BLOCK_SIZE,
            ]);

            // Check for any gold to pick up.
            gold_piles.retain(|gold_pile| {
                let xd = x - (gold_pile.position[0] + 35.0 / 2.0);
                let yd = y - (gold_pile.position[1] + 35.0 / 2.0);
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
            self.selected_tile_location = None;
        }
    }
}
