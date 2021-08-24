use crate::asset_system::AssetManager;
use crate::game_components::{
    monsters::MonsterState,
    towers::{BasicTower, NinjaTower, TowerType},
    Board, Player, BLOCK_SIZE,
};
use crate::{level_system::monster_spawner::MonsterSpawner, ui_system::ui::*, utils::Scale};

use ggez::{
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics, Context, GameResult,
};

use log::debug;

use std::time;

pub struct MainState {
    asset_manager: AssetManager,
    player: Player,
    monster_spawner: MonsterSpawner,
    ui: UI,
    board: Board,
    time: time::Instant,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        MainState {
            asset_manager: AssetManager::new(ctx),
            player: Player {
                health: 100.0,
                gold: 300,
            },
            monster_spawner: MonsterSpawner::new(),
            ui: UI::new(),
            board: Board::generate(1, 2),
            time: time::Instant::now(),
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _: &mut ggez::Context) -> std::result::Result<(), ggez::GameError> {
        let elapsed = self.time.elapsed().as_millis() as f32 / 1000.0;
        debug!("MainState: update: elapsed{}", elapsed);

        self.monster_spawner.update(elapsed, &mut self.board);

        for monster_view in self.board.monster_views.iter_mut() {
            monster_view.get_monster_mut().update(
                elapsed,
                &self.board.path_blocks,
                &mut self.player,
            );
        }

        debug!(
            "MainState: update: monsters length before removing dead monsters: {}",
            self.board.monster_views.len()
        );
        self.board
            .monster_views
            .retain(|x| x.get_monster().get_current_state() != MonsterState::Dead);
        debug!(
            "MainState: update: monsters length after removing dead monsters: {}",
            self.board.monster_views.len()
        );

        for tower in self.board.towers.iter_mut() {
            tower.update(
                elapsed,
                &mut self.board.monster_views,
                &mut self.board.gold_piles,
                &mut self.asset_manager,
            );
        }
        self.time = time::Instant::now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        debug!("MainState: draw: drawing path blocks.");
        for block in self.board.path_blocks.iter_mut() {
            block.draw(ctx)?;
        }

        debug!("MainState: draw: drawing monsters.");
        for monster_view in self.board.monster_views.iter_mut() {
            monster_view.draw(ctx, &self.asset_manager)?;
        }

        debug!("MainState: draw: drawing towers.");
        for tower in self.board.towers.iter_mut() {
            tower.draw(ctx, &self.asset_manager)?;
        }

        debug!("MainState: draw: drawing tower attacks.");
        // Draw tower attacks.
        for tower in self.board.towers.iter_mut() {
            tower.draw_abilities(ctx, &self.board.monster_views)?;
        }

        debug!("MainState: draw: drawing gold piles.");
        for gold_pile in self.board.gold_piles.iter_mut() {
            gold_pile.draw(ctx, &self.asset_manager)?;
        }

        debug!("MainState: draw: drawing base.");
        self.board.base.draw(ctx, &self.asset_manager)?;

        debug!("MainState: draw: drawing base.");
        self.ui.draw(ctx, &self.player, &self.asset_manager)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.ui.mouse_motion_event(
            ctx,
            x,
            y,
            &mut self.board.gold_piles,
            &mut self.player,
            &mut self.asset_manager,
        );
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        debug!(
            "MainState: mouse_button_down_event: button({:?}), x({}), y({}).",
            _button, x, y
        );

        if let Some(tower_type) = self.ui.hovering_on {
            self.ui.selected_tile_type = tower_type;
        }

        if let Some(_) = self.ui.selected_tile_rect {
            let screen_rect = graphics::drawable_size(ctx);

            let scale = Scale {
                x: screen_rect.0 / 800.0, // 800.0 default width.
                y: screen_rect.1 / 600.0, // 600.0 default height.
            };
            let scaled_position = scale.to_game_point(x, y);

            // Check that position is clear.
            if !self
                .board
                .position_is_occupied([scaled_position.x, scaled_position.y])
            {
                let block_position = [
                    (scaled_position.x / BLOCK_SIZE).floor(),
                    (scaled_position.y / BLOCK_SIZE).floor(),
                ];

                if self.ui.selected_tile_type == TowerType::Basic {
                    if self.player.gold >= 10 {
                        self.player.gold -= 10;
                        debug!("MainState: mouse_button_down_event: placing new BasicTower at x({}), y({}).", block_position[0], block_position[1]);
                        self.board
                            .add_tower(Box::new(BasicTower::new(block_position)));
                    }
                } else if self.ui.selected_tile_type == TowerType::Ninja {
                    if self.player.gold >= 20 {
                        self.player.gold -= 20;
                        debug!("MainState: mouse_button_down_event: placing new NinjaTower at x({}), y({}).", block_position[0], block_position[1]);
                        self.board
                            .add_tower(Box::new(NinjaTower::new(block_position)));
                    }
                }
            }
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        debug!(
            "MainState: key_down_event: keycode({:?}), keymods({:?}), repeat({})",
            keycode, _keymods, _repeat
        );

        if keycode == KeyCode::Key1 {
            debug!("MainState: key_down_event: switching to TowerType::Basic.");
            self.ui.selected_tile_type = TowerType::Basic;
        } else if keycode == KeyCode::Key2 {
            debug!("MainState: key_down_event: switching to TowerType::Ninja.");
            self.ui.selected_tile_type = TowerType::Ninja;
        }
    }
}
