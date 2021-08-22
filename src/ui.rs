use crate::asset_manager::AssetManager;
use crate::gold::GoldPile;
use crate::tower_icon::{TowerIcon, TOWER_ICON_SIZE};
use crate::towers::tower::TowerType;
use crate::utils::Scale;
use crate::{Player, BLOCK_SIZE};

use ggez::graphics::{self, DrawParam};
use ggez::mint::Point2;
use ggez::graphics::Rect;
use ggez::{audio::SoundSource, Context, GameResult};

pub const UI_HEIGHT: f32 = 180.0;

const BUILD_BAR_POSITION: Point2<f32> = Point2 { x: 180.0, y: 10.0 };

const GOLD_X: f32 = 30.0;
const GOLD_Y: f32 = 30.0;

const HP_X: f32 = 30.0;
const HP_Y: f32 = 50.0;

pub struct UI {
    position: Point2<f32>,
    rect: Rect,
    pub build_bar: Vec<TowerIcon>,
    pub hovering_on: Option<TowerType>,
    pub selected_tile_rect: Option<[f32; 2]>,
    pub selected_tile_type: TowerType,
}

/// UI responsible for drawing the status bar and build bar. All
/// drawing/positioning should be relative to the UI location:
/// (0, screen_height-UI_HEIGHT)
/// and scaled.
impl UI {
    pub fn new() -> UI {
        UI {
            position: Point2 { x: 0.0, y: 0.0 },
            rect: Rect { x: 0.0, y: 0.0, w: 0.0, h: 0.0 },
            build_bar: vec!(
                TowerIcon { tower_type: TowerType::Basic },
                TowerIcon { tower_type: TowerType::Ninja },
            ),
            hovering_on: None,
            selected_tile_rect: None,
            selected_tile_type: TowerType::Basic,
        }
    }

    fn update_position_and_size(&mut self, ctx: &Context) {
        let screen_size = ggez::graphics::drawable_size(ctx);

        let scale = Scale {
            x: screen_size.0 / 800.0,
            y: screen_size.1 / 600.0,
        };
        self.rect = [0.0, 0.0, screen_size.0 / scale.x, UI_HEIGHT / scale.y].into();
        self.position = Point2 {
            x: 0.0,
            y: (screen_size.1 - UI_HEIGHT) / scale.y,
        };
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        player: &Player,
        asset_manager: &AssetManager,
    ) -> GameResult {
        self.update_position_and_size(ctx);

        self.draw_background(ctx)?;
        self.draw_gold(ctx, player)?;
        self.draw_hp(ctx, player)?;
        self.draw_build_bar(ctx, asset_manager)?;
        self.draw_selected_tile(ctx)?;
        Ok(())
    }

    fn draw_background(&mut self, ctx: &mut Context) -> GameResult {
        // Divide width/height by scale to prevent ggez resize from changing
        // the relative size of the UI (100% width).
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.rect,
            ggez::graphics::Color::new(0.2, 0.3, 0.4, 1.0),
        )?;

        graphics::draw(
            ctx,
            &rectangle,
            DrawParam::default()
                .dest(self.position)
        )?;
        Ok(())
    }

    fn draw_gold(&mut self, ctx: &mut Context, player: &Player) -> GameResult {
        let text = graphics::Text::new(format!("GOLD: {}", player.gold));
        let location_x = GOLD_X;
        let location_y = self.position.y + GOLD_Y;
        let location = (Point2 {
            x: location_x,
            y: location_y
        },);
        graphics::draw(ctx, &text, location)?;
        Ok(())
    }

    fn draw_hp(&mut self, ctx: &mut Context, player: &Player) -> GameResult {
        let text = graphics::Text::new(format!("HP: {}", player.health));
        let location_x = HP_X;
        let location_y = self.position.y + HP_Y;
        let location = (Point2 {
            x: location_x,
            y: location_y
        },);
        graphics::draw(ctx, &text, location)?;
        Ok(())
    }

    /// Draws all tower icons inside the build_bar which can fit inside the UI.
    /// Overflowing tower icons are not drawn.
    fn draw_build_bar(
        &mut self,
        ctx: &mut Context,
        asset_manager: &AssetManager,
    ) -> GameResult {
        let mut offset = Point2 {
            x: BUILD_BAR_POSITION.x,
            y: self.position.y + BUILD_BAR_POSITION.y,
        };
        debug!("draw_build_bar: build_bar offset: {:?}", offset);

        debug!("draw_build_bar: drawing tower icons.");
        for tower in self.build_bar.iter_mut() {
            tower.draw(ctx, asset_manager, offset, self.hovering_on == Some(tower.tower_type))?;
            // Only tiles in x direction for now.
            offset.x += TOWER_ICON_SIZE;
        }

        Ok(())
    }

    fn draw_selected_tile(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(tile) = self.selected_tile_rect {
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
        ctx: &Context,
        scale: Scale,
        x: f32,
        y: f32,
        gold_piles: &mut Vec<GoldPile>,
        player: &mut Player,
        asset_manager: &mut AssetManager,
    ) {
        debug!("mouse_motion_event: {:?} {:?} {:?} {:?}", scale, x, y, gold_piles.len());
        let screen_rect = ggez::graphics::screen_coordinates(ctx);

        // Check inside game window.
        if x > 0.0 && x < screen_rect.w && y > 0.0 && y < screen_rect.h - UI_HEIGHT * scale.y {
            self.handle_in_game_hover(
                scale,
                x,
                y,
                gold_piles,
                player,
                asset_manager,
            );
        } else {
            self.handle_ui_bar_hover( ctx, scale, x, y, );
        }
    }

    pub fn handle_in_game_hover(
        &mut self,
        scale: Scale,
        x: f32,
        y: f32,
        gold_piles: &mut Vec<GoldPile>,
        player: &mut Player,
        asset_manager: &mut AssetManager,
    ) {
        debug!("handle_in_game_hover");
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
                asset_manager.item_assets.gold_sound.play().unwrap();
                false
            } else {
                true
            }
        });
    }

    pub fn handle_ui_bar_hover(
        &mut self,
        ctx: &Context,
        scale: Scale,
        x: f32,
        y: f32,
    ) {
        debug!("handle_ui_bar_hover");
        let screen_size = ggez::graphics::drawable_size(ctx);

        self.selected_tile_rect = None;

        // Check if hovering over build bar.
        let build_bar_position = Point2 {
            x: BUILD_BAR_POSITION.x,
            y: screen_size.1 - UI_HEIGHT + BUILD_BAR_POSITION.y,
        };

        let vp_build_bar_position = scale.to_viewport_point(
            build_bar_position.x,
            build_bar_position.y
        );

        // Dynamically scales in x-direction (or so is the idea) so can't
        // statically check the right and bottom boundery.
        debug!("scale: {:?}", scale);
        if x > vp_build_bar_position.x && y > vp_build_bar_position.y {
            // Check for each tower icon if the mouse is hovering over it.
            let mut cumulative_width = vp_build_bar_position.x;

            // All tower icons have the same size.
            for tower in self.build_bar.iter() {
                // Mouse is inside tower icon.
                if x > cumulative_width
                    && x < cumulative_width + TOWER_ICON_SIZE * scale.x
                    && y > vp_build_bar_position.y * scale.y
                    && y < vp_build_bar_position.y + TOWER_ICON_SIZE * scale.y {
                    self.hovering_on = Some(tower.tower_type);
                    return;
                }
                cumulative_width += TOWER_ICON_SIZE * scale.x;
            }
        }

        // Mouse is not hovering over any tower icon, clear the hover.
        self.hovering_on = None;
    }
}
