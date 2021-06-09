use crate::{
    block::BLOCK_SIZE,
    asset_manager::AssetManager,
    player::Player,
    ui::{ UI, WINDOW_HEIGHT, WINDOW_WIDTH, UI_HEIGHT, },
    monster::MonsterState,
    tower::Tower,
    monster_spawner::MonsterSpawner,
    board::Board,
};

use ggez::{Context, GameResult, event::{self, EventHandler}, graphics};

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
                gold: 100,
            },
            monster_spawner: MonsterSpawner::new(),
            ui: UI {
                build_bar: Vec::new(),
                selected_tile: None,
            },
            board: Board::generate(1, 2),
            time: time::Instant::now(),
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _: &mut ggez::Context) -> std::result::Result<(), ggez::GameError> {
        let elapsed = self.time.elapsed().as_millis() as f32 / 1000.0;

        self.monster_spawner.update(elapsed, &mut self.board);

        for monster in self.board.monsters.iter_mut() {
            monster.update(elapsed, &self.board.path_blocks, &mut self.player);
        }
        self.board
            .monsters
            .retain(|x| x.state != MonsterState::Dead);

        for tower in self.board.towers.iter_mut() {
            tower.update(elapsed, &mut self.board.monsters, &mut self.asset_manager);
        }
        self.time = time::Instant::now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for block in self.board.path_blocks.iter_mut() {
            block.draw(ctx)?;
        }

        for monster in self.board.monsters.iter_mut() {
            monster.draw(ctx, &self.asset_manager)?;
        }

        for tower in self.board.towers.iter_mut() {
            tower.draw(ctx, &self.board.monsters, &self.asset_manager)?;
        }

        // Draw tower attacks.
        for tower in self.board.towers.iter_mut() {
            tower.draw_attacks(ctx, &self.board.monsters)?;
        }

        self.board.base.draw(ctx)?;
        self.ui.draw(ctx, &self.player)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        // Check inside game window.
        if x > 0.0 && x < WINDOW_WIDTH && y > 0.0 && y < WINDOW_HEIGHT - UI_HEIGHT {
            let xd = (x / BLOCK_SIZE).floor() * BLOCK_SIZE;
            let yd = (y / BLOCK_SIZE).floor() * BLOCK_SIZE;

            // Change selected_tile.
            self.ui.selected_tile = Some((xd, yd));
        } else {
            self.ui.selected_tile = None;
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if let Some(tile) = self.ui.selected_tile {
            if self.player.gold >= 10 {
                self.player.gold -= 10;
                self.board.towers.push(Tower::new([
                    (x / BLOCK_SIZE).floor(),
                    (y / BLOCK_SIZE).floor(),
                ]))
            }
        }
    }
}
