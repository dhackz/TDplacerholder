use ggez::{Context, GameResult, audio::{self, SoundSource}, event::{self, EventHandler}, graphics};

use std::path;
use std::env;
use std::time;

const BLOCK_SIZE : f32 = 35.0;
const BASE_SIZE : f32 = 60.0;
const BASE_PADDING: f32 = 5.0;

const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_WIDTH: f32 = 800.0;
const UI_HEIGHT: f32 = 180.0;

const GOLD_X: f32 = 30.0;
const GOLD_Y: f32 = 30.0;

const HP_X: f32 = 30.0;
const HP_Y: f32 = 50.0;

pub struct MonsterSpawner {
    spawn_schedule: Vec<f32>,
    elapsed_time: f32,
}

impl MonsterSpawner {
    pub fn new() -> MonsterSpawner {
        let spawn_schedule = vec![ 0.0, 1.0, 2.0, 3.0, 4.0, ];
        MonsterSpawner { spawn_schedule, elapsed_time: 0.0 }
    }

    pub fn update(&mut self, elapsed: f32, board: &mut Board) {
        self.elapsed_time += elapsed;

        for i in 0..self.spawn_schedule.len() {
            if self.spawn_schedule[i] < self.elapsed_time {
                board.monsters.push(
                    Monster::new_basic_monster()
                );
                if i == self.spawn_schedule.len()-1 {
                    self.spawn_schedule = vec![];
                }
            } else {
                self.spawn_schedule = self.spawn_schedule.split_off(i);
                break; // Schedule is cronological, no reason to check further.
            }
        }

    }
}

pub struct AssetManager {
    tower_sprite: graphics::Image,
    tower_attack_sound: audio::Source,
    monster_sprite: graphics::Image,
    // monster_hurt_sound: graphics::Image,
}

impl AssetManager {
    pub fn new(ctx: &mut Context) -> AssetManager {
        AssetManager {
            tower_sprite: graphics::Image::new(ctx, "/tower2.png").unwrap(),
            tower_attack_sound: audio::Source::new(ctx, "/tower_attack.ogg").unwrap(),
            monster_sprite: graphics::Image::new(ctx, "/monster1.png").unwrap(),
        }
    }
}

pub struct Player {
    health: f32,
    gold: u32,
}

pub struct Block {
    pos: (f32, f32),
}

impl Block {
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE].into(),
            ggez::graphics::Color::new(1.0, 1.0, 0.0, 1.0),
        )?;

        let location = (
            ggez::mint::Point2 {
                x: self.pos.0 * BLOCK_SIZE,
                y: self.pos.1 * BLOCK_SIZE,
            },
        );

        graphics::draw(ctx, &rectangle, location)?;
        Ok(())
    }
}

pub struct Base { pos: (f32, f32) }

impl Base {
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, BASE_SIZE, BASE_SIZE].into(),
            ggez::graphics::Color::new(0.0, 1.0, 0.0, 1.0),
        )?;

        let location = (
            ggez::mint::Point2 {
                x: self.pos.0 * BLOCK_SIZE + BASE_PADDING,
                y: self.pos.1 * BLOCK_SIZE + BASE_SIZE/2.0 - BASE_PADDING,
            },
        );

        graphics::draw(ctx, &rectangle, location)?;
        Ok(())
    }
}

struct Tower {
    position: [f32; 2],
    attack_cooldown: f32,
}

impl Tower {
    const ATTACK_RANGE: f32 = 100.0;  // Pixels.
    const ATTACK_TIMER: f32 = 1.0; // Seconds.
    const DAMAGE: f32 = 10.0;

    pub fn new(position: [f32; 2]) -> Tower {
        Tower {
            position,
            attack_cooldown: 0.0,
        }
    }

    pub fn get_center_pos_abs(&self) -> [f32; 2] {
        [
            self.position[0] * BLOCK_SIZE + BLOCK_SIZE/2.0,
            self.position[1] * BLOCK_SIZE + BLOCK_SIZE/2.0,
        ]
    }

    fn position_is_in_attack_range(&self, position_abs: [f32; 2]) -> bool {
        let tower_center_pos_abs = self.get_center_pos_abs();

        let dx = tower_center_pos_abs[0] - position_abs[0];
        let dy = tower_center_pos_abs[1] - position_abs[1];

        dx*dx + dy*dy < Tower::ATTACK_RANGE*Tower::ATTACK_RANGE
    }

    pub fn update(
        &mut self,
        elapsed: f32,
        monsters: &mut Vec<Monster>,
        asset_manager: &mut AssetManager
    ) {
        self.attack_cooldown -= elapsed;

        if self.attack_cooldown < 0.0 {
            self.attack_cooldown = 0.0;
        }

        if self.attack_cooldown == 0.0 {
            let mut damage_dealt = false;
            for monster in monsters.iter_mut() {
                if self.position_is_in_attack_range(monster.get_center_pos_abs()) {
                    damage_dealt = true;
                    monster.recieve_damage(Tower::DAMAGE);
                }
            }
            if damage_dealt {
                asset_manager.tower_attack_sound.play().unwrap();
                self.attack_cooldown = Tower::ATTACK_TIMER;
            }
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        monsters: &Vec<Monster>,
        asset_manager: &AssetManager
    ) -> GameResult {
        let location = (
            ggez::mint::Point2 {
                x: self.position[0] * BLOCK_SIZE - 5.0,
                y: self.position[1] * BLOCK_SIZE - 35.0,
            },
        );

        graphics::draw(ctx, &asset_manager.tower_sprite, location)?;

        Ok(())
    }

    pub fn draw_attacks(&mut self, ctx: &mut Context, monsters: &Vec<Monster>) -> GameResult {
        for monster in monsters.iter() {
            let monster_center = [
                monster.position[0] + Monster::SIZE/2.0,
                monster.position[1] + Monster::SIZE/2.0,
            ];

            if self.position_is_in_attack_range(monster_center) {
                self.draw_attack(ctx, self.get_center_pos_abs(), monster_center);
            }
        }
        Ok(())
    }

    fn draw_attack(
        &mut self,
        ctx: &mut Context,
        from_abs: [f32; 2],
        to_abs: [f32; 2]
    ) -> GameResult {
        let line = graphics::Mesh::new_line(
            ctx,
            &[from_abs, to_abs],
            3.0,
            ggez::graphics::Color::new(0.0, 1.0, 1.0, 1.0),
        )?;

        let location = (
            ggez::mint::Point2 {
                x: 0.0,
                y: 0.0,
            },
        );

        graphics::draw(ctx, &line, location)?;
        Ok(())
    }
}

pub struct TowerIcon {}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
pub enum MonsterState // TODO: put in namespace.
{
    Walking,
    Attacking,
    Dead,
}

pub struct Monster {
    position: [f32; 2],
    speed: f32,
    health: f32,
    move_goal: usize,
    state: MonsterState,
}

impl Monster {
    const SIZE: f32 = 20.0;
    const DAMAGE: f32 = 1.0;

    pub fn new_basic_monster() -> Monster {
        Monster {
            health: 100.0,
            speed: 100.0,
            position: [0.0, 0.0],
            move_goal: 0,
            state: MonsterState::Walking,
        }
    }

    pub fn get_center_pos_abs(&self) -> [f32; 2] {
        [self.position[0] + Monster::SIZE/2.0, self.position[1] + Monster::SIZE/2.0]
    }

    pub fn recieve_damage(&mut self, damage: f32) {
        self.health -= damage;

        if self.health <= 0.0 {
            self.state = MonsterState::Dead;
        }
    }

    /// Try moving towards the currently targeted path block position.
    fn try_moving(&mut self, elapsed: f32, path_blocks: &Vec<Block>) {
        // Don't move unless in walking state.
        if self.state != MonsterState::Walking { return }

        // Check if at end of path.
        if self.move_goal == path_blocks.len() {
            self.state = MonsterState::Attacking;
            return; // No more moving to do.

            // Queue damage on player.
            // Mark as dead.
            // Enter Monster_dealt_damage state&wait for cleanup?
        }

        // Goal is for center of monster to pass center of block position.
        let _goal = path_blocks[self.move_goal].pos;
        let goal_x = _goal.0 * BLOCK_SIZE + BLOCK_SIZE/2.0 - Monster::SIZE/2.0;
        let goal_y = _goal.1 * BLOCK_SIZE + BLOCK_SIZE/2.0 - Monster::SIZE/2.0;
        let goal = (goal_x, goal_y);

        // Distance to next goal position.
        let mut dir = (goal.0 - self.position[0], goal.1 - self.position[1]);
        let mut dist = dir.0*dir.0+dir.1*dir.1;

        // Special case where we are exactly at the right position.
        if dist == 0.0 {
            self.move_goal += 1;
        } else if dist > 0.0 {
            // We have not yet reached the goal destination.

            // Normailze the direction vector so it doesn't scale the speed.
            dist = dist.sqrt();
            dir.0 = dir.0/dist;
            dir.1 = dir.1/dist;

            // If 1 step is too far/we pass the goal only move to the goal
            // position/don't overshoot.
            if dist < self.speed*elapsed {
                self.move_goal += 1;

                self.position[0] += dir.0*dist;
                self.position[1] += dir.1*dist;
            } else {
                // 1 step will not reach the goal.
                self.position[0] += dir.0*self.speed*elapsed;
                self.position[1] += dir.1*self.speed*elapsed;
            }
        }
    }

    fn update(&mut self, elapsed: f32, path_blocks: &Vec<Block>, player: &mut Player) {
        if self.state == MonsterState::Attacking {
            // Die and deal damange to the player.
            player.health -= Monster::DAMAGE;
            self.state = MonsterState::Dead;
        }

        // Don't do anything if dead.
        if self.state == MonsterState::Dead {
            return;
        }

        self.try_moving(elapsed, path_blocks);
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        asset_manager: &AssetManager
    ) -> GameResult {
        let location = (
            ggez::mint::Point2 {
                x: self.position[0],
                y: self.position[1] - 10.0,
            },
        );

        graphics::draw(ctx, &asset_manager.monster_sprite, location)?;

        Ok(())
    }
}

pub struct Board {
    path_blocks: Vec<Block>,
    towers: Vec<Tower>,
    monsters: Vec<Monster>,
    base: Base,
}

impl Board {
    fn generate(seed: u64, length: u32) -> Board {
        let mut path_blocks = Vec::new();
        path_blocks.push( Block { pos: (0.0, 0.0) } );
        path_blocks.push( Block { pos: (0.0, 1.0) } );
        path_blocks.push( Block { pos: (0.0, 2.0) } );
        path_blocks.push( Block { pos: (1.0, 2.0) } );
        path_blocks.push( Block { pos: (2.0, 2.0) } );
        path_blocks.push( Block { pos: (2.0, 3.0) } );
        path_blocks.push( Block { pos: (2.0, 3.0) } );
        path_blocks.push( Block { pos: (2.0, 4.0) } );
        path_blocks.push( Block { pos: (2.0, 5.0) } );
        path_blocks.push( Block { pos: (3.0, 5.0) } );
        path_blocks.push( Block { pos: (4.0, 5.0) } );
        path_blocks.push( Block { pos: (5.0, 5.0) } );
        path_blocks.push( Block { pos: (6.0, 5.0) } );
        path_blocks.push( Block { pos: (7.0, 5.0) } );
        path_blocks.push( Block { pos: (8.0, 5.0) } );
        path_blocks.push( Block { pos: (9.0, 5.0) } );
        path_blocks.push( Block { pos: (10.0, 5.0) } );
        path_blocks.push( Block { pos: (11.0, 5.0) } );
        path_blocks.push( Block { pos: (12.0, 5.0) } );
        path_blocks.push( Block { pos: (13.0, 5.0) } );
        path_blocks.push( Block { pos: (14.0, 5.0) } );
        path_blocks.push( Block { pos: (15.0, 5.0) } );
        path_blocks.push( Block { pos: (16.0, 5.0) } );
        path_blocks.push( Block { pos: (17.0, 5.0) } );
        path_blocks.push( Block { pos: (18.0, 5.0) } );
        path_blocks.push( Block { pos: (19.0, 5.0) } );
        path_blocks.push( Block { pos: (20.0, 5.0) } );
        path_blocks.push( Block { pos: (20.0, 6.0) } );
        path_blocks.push( Block { pos: (20.0, 7.0) } );
        path_blocks.push( Block { pos: (20.0, 8.0) } );
        path_blocks.push( Block { pos: (20.0, 9.0) } );
        path_blocks.push( Block { pos: (19.0, 9.0) } );
        path_blocks.push( Block { pos: (18.0, 9.0) } );
        path_blocks.push( Block { pos: (17.0, 9.0) } );
        path_blocks.push( Block { pos: (16.0, 9.0) } );
        path_blocks.push( Block { pos: (15.0, 9.0) } );
        path_blocks.push( Block { pos: (14.0, 9.0) } );
        path_blocks.push( Block { pos: (13.0, 9.0) } );
        path_blocks.push( Block { pos: (12.0, 9.0) } );
        path_blocks.push( Block { pos: (11.0, 9.0) } );
        path_blocks.push( Block { pos: (10.0, 9.0) } );
        path_blocks.push( Block { pos: (9.0, 9.0) } );
        path_blocks.push( Block { pos: (8.0, 9.0) } );
        path_blocks.push( Block { pos: (7.0, 9.0) } );
        path_blocks.push( Block { pos: (6.0, 9.0) } );
        path_blocks.push( Block { pos: (5.0, 9.0) } );
        path_blocks.push( Block { pos: (4.0, 9.0) } );
        path_blocks.push( Block { pos: (3.0, 9.0) } );
        path_blocks.push( Block { pos: (2.0, 9.0) } );

        Board {
            path_blocks,
            towers: Vec::new(),
            monsters: Vec::new(),
            base: Base { pos: (0.0, 8.0) }
        }
    }
}

pub struct UI {
    build_bar: Vec<TowerIcon>,
    selected_tile: Option<(f32, f32)>,
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

        let location = (
            ggez::mint::Point2 {
                x: 0.0,
                y: WINDOW_HEIGHT - UI_HEIGHT,
            },
        );
        graphics::draw(ctx, &rectangle, location)?;
        Ok(())
    }

    fn draw_gold(&mut self, ctx: &mut Context, player: &Player) -> GameResult {
        let text = graphics::Text::new(format!("GOLD: {}", player.gold));
        let location_x = GOLD_X;
        let location_y = WINDOW_HEIGHT - UI_HEIGHT + GOLD_Y;
        let location = (
            ggez::mint::Point2 {
                x: location_x,
                y: location_y,
            },
        );
        graphics::draw(ctx, &text, location)?;
        Ok(())
    }

    fn draw_hp(&mut self, ctx: &mut Context, player: &Player) -> GameResult {
        let text = graphics::Text::new(format!("HP: {}", player.health));
        let location_x = HP_X;
        let location_y = WINDOW_HEIGHT - UI_HEIGHT + HP_Y;
        let location = (
            ggez::mint::Point2 {
                x: location_x,
                y: location_y,
            },
        );
        graphics::draw(ctx, &text, location)?;
        Ok(())
    }

    fn draw_selected_tile(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(tile) = self.selected_tile {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(3.0),
                [0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE].into(),
                ggez::graphics::Color::new(0.5, 0.0, 0.0, 1.0),
            )?;

            let location = (
                ggez::mint::Point2 {
                    x: tile.0,
                    y: tile.1,
                },
            );
            graphics::draw(ctx, &rectangle, location)?;
        }
        Ok(())
    }
}

pub struct MainState {
    asset_manager: AssetManager,
    player: Player,
    monster_spawner: MonsterSpawner,
    ui: UI,
    board: Board,
    time: time::Instant,
}

impl MainState {
    fn new(ctx: &mut Context) -> MainState {
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
            board: Board::generate(1,2),
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
        self.board.monsters.retain(|x| x.state != MonsterState::Dead);

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

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) {
        // Check inside game window.
        if x > 0.0 && x < WINDOW_WIDTH &&
            y > 0.0 && y < WINDOW_HEIGHT - UI_HEIGHT {
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
                self.board.towers.push(
                    Tower::new([(x/BLOCK_SIZE).floor(), (y/BLOCK_SIZE).floor()]),
                )
            }
        }
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_file) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_file);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("TowerOfDerp", "rrEd").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx);
    event::run(ctx, event_loop, state)
}
