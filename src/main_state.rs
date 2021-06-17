use crate::{
    asset_manager::AssetManager,
    block::BLOCK_SIZE,
    board::Board,
    monsters::monster::MonsterState,
    monster_spawner::MonsterSpawner,
    player::Player,
    towers::{basic_tower::*, ninja_tower::*},
    ui::*,
};

use log::debug;

use ggez::{
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics, Context, GameResult,
};

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
                gold: 30,
            },
            monster_spawner: MonsterSpawner::new(),
            ui: UI {
                build_bar: Vec::new(),
                selected_tile_location: None,
                selected_tile_type: TowerType::Basic,
            },
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

        for monster in self.board.monsters.iter_mut() {
            monster.update(elapsed, &self.board.path_blocks, &mut self.player);
        }

        debug!(
            "MainState: update: monsters length before removing dead monsters: {}",
            self.board.monsters.len()
        );
        self.board
            .monsters
            .retain(|x| x.get_current_state() != MonsterState::Dead);
        debug!(
            "MainState: update: monsters length after removing dead monsters: {}",
            self.board.monsters.len()
        );

        for tower in self.board.towers.iter_mut() {
            tower.update(
                elapsed,
                &mut self.board.monsters,
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
        for monster in self.board.monsters.iter_mut() {
            monster.draw(ctx, &self.asset_manager)?;
        }

        debug!("MainState: draw: drawing towers.");
        for tower in self.board.towers.iter_mut() {
            tower.draw(ctx, &self.asset_manager)?;
        }

        debug!("MainState: draw: drawing tower attacks.");
        // Draw tower attacks.
        for tower in self.board.towers.iter_mut() {
            tower.draw_abilities(ctx, &self.board.monsters)?;
        }

        debug!("MainState: draw: drawing gold piles.");
        for gold_pile in self.board.gold_piles.iter_mut() {
            gold_pile.draw(ctx, &self.asset_manager)?;
        }

        debug!("MainState: draw: drawing base.");
        self.board.base.draw(ctx, &self.asset_manager)?;
        debug!("MainState: draw: drawing base.");
        self.ui.draw(ctx, &self.player)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.ui.mouse_motion_event(
            x,
            y,
            &mut self.board.gold_piles,
            &mut self.player,
            &mut self.asset_manager,
        );
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        debug!(
            "MainState: mouse_button_down_event: button({:?}), x({}), y({}).",
            _button, x, y
        );

        if let Some(_) = self.ui.selected_tile_location {
            if self.player.gold >= 10 {
                self.player.gold -= 10;
                if self.ui.selected_tile_type == TowerType::Basic {
                    debug!("MainState: mouse_button_down_event: placing new BasicTower at x({}), y({}).", (x / BLOCK_SIZE).floor(), (y / BLOCK_SIZE).floor());
                    self.board.towers.push(Box::new(BasicTower::new([
                        (x / BLOCK_SIZE).floor(),
                        (y / BLOCK_SIZE).floor(),
                    ])));
                } else if self.ui.selected_tile_type == TowerType::Ninja {
                    debug!("MainState: mouse_button_down_event: placing new NinjaTower at x({}), y({}).", (x / BLOCK_SIZE).floor(), (y / BLOCK_SIZE).floor());
                    self.board.towers.push(Box::new(NinjaTower::new([
                        (x / BLOCK_SIZE).floor(),
                        (y / BLOCK_SIZE).floor(),
                    ])));
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
