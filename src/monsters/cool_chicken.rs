use crate::monsters::monster::{Monster, MonsterState};
use crate::{asset_manager::AssetManager, gold::GoldPile, Block, Player, BLOCK_SIZE};

use ggez::{
    audio::SoundSource,
    graphics::{self, DrawParam},
    Context, GameResult,
};

use rand::*;

#[derive(Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

pub struct CoolChicken {
    pub position: [f32; 2],
    pub speed: f32,
    pub health: f32,
    pub move_goal: usize,
    pub state: MonsterState,
    pub direction: Direction,
}

impl CoolChicken {
    pub const SIZE: f32 = 20.0;
    pub const DAMAGE: f32 = 1.0;

    pub fn new() -> CoolChicken {
        CoolChicken {
            health: 100.0,
            speed: 100.0,
            position: [0.0, 0.0],
            move_goal: 0,
            state: MonsterState::Walking,
            direction: Direction::Right,
        }
    }

    /// Try moving towards the currently targeted path block position.
    fn try_moving(&mut self, elapsed: f32, path_blocks: &Vec<Block>) {
        // Don't move unless in walking state.
        if self.state != MonsterState::Walking {
            return;
        }

        // Check if at end of path.
        if self.move_goal == path_blocks.len() {
            self.state = MonsterState::Attacking;
            return; // No more moving to do.

            // Queue damage on player.
            // Mark as dead.
            // Enter Monster_dealt_damage state&wait for cleanup?
        }

        // Goal is for center of monster to pass center of block position.
        let _goal = path_blocks[self.move_goal].position;
        let goal_x = _goal[0] * BLOCK_SIZE + BLOCK_SIZE / 2.0 - CoolChicken::SIZE / 2.0;
        let goal_y = _goal[1] * BLOCK_SIZE + BLOCK_SIZE / 2.0 - CoolChicken::SIZE / 2.0;
        let goal = [goal_x, goal_y];

        // Distance to next goal position.
        let mut dir = [goal[0] - self.position[0], goal[1] - self.position[1]];
        let mut dist = dir[0] * dir[0] + dir[1] * dir[1];

        if dir[0] >= 0.0 {
            self.direction = Direction::Right;
        } else {
            self.direction = Direction::Left;
        }

        // Special case where we are exactly at the right position.
        if dist == 0.0 {
            self.move_goal += 1;
        } else if dist > 0.0 {
            // We have not yet reached the goal destination.

            // Normailze the direction vector so it doesn't scale the speed.
            dist = dist.sqrt();
            dir[0] = dir[0] / dist;
            dir[1] = dir[1] / dist;

            // If 1 step is too far/we pass the goal only move to the goal
            // position/don't overshoot.
            if dist < self.speed * elapsed {
                self.move_goal += 1;

                self.position[0] += dir[0] * dist;
                self.position[1] += dir[1] * dist;
            } else {
                // 1 step will not reach the goal.
                self.position[0] += dir[0] * self.speed * elapsed;
                self.position[1] += dir[1] * self.speed * elapsed;
            }
        }
    }
}

impl Monster for CoolChicken {
    fn get_center_pos_abs(&self) -> [f32; 2] {
        [
            self.position[0] + CoolChicken::SIZE / 2.0,
            self.position[1] + CoolChicken::SIZE / 2.0,
        ]
    }

    fn recieve_damage(
        &mut self,
        damage: f32,
        gold_piles: &mut Vec<GoldPile>,
        asset_manager: &mut AssetManager,
    ) {
        if self.state == MonsterState::Dead {
            // Already dead do nothing.
            return;
        }

        self.health -= damage;

        if self.health <= 0.0 {
            asset_manager.monster_hurt_sound.play().unwrap();
            self.state = MonsterState::Dead;

            let offset = 10.0;
            let mut rng = rand::thread_rng();

            let gold_position = [
                self.position[0] + (rng.gen::<f32>() * offset - offset * 2.0),
                self.position[1] + (rng.gen::<f32>() * offset - offset * 2.0),
            ];

            gold_piles.push(GoldPile {
                position: gold_position,
                value: 10,
            });
        }
    }

    fn update(&mut self, elapsed: f32, path_blocks: &Vec<Block>, player: &mut Player) {
        if self.state == MonsterState::Attacking {
            // Die and deal damange to the player.
            player.health -= CoolChicken::DAMAGE;
            self.state = MonsterState::Dead;
        }

        // Don't do anything if dead.
        if self.state == MonsterState::Dead {
            return;
        }

        self.try_moving(elapsed, path_blocks);
    }

    fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult {
        let half_width = asset_manager.cool_chicken_sprite.width() as f32 / 2.0;
        let half_height = asset_manager.cool_chicken_sprite.height() as f32 / 2.0;

        if self.direction == Direction::Left {
            // Flipping along y-axis causes image to end up at a position
            // (-width, 0). Offsetting with (+width/2, -height/2) makes the
            // image center end up at (0,0).
            let offset_position = [
                self.position[0] + half_width,
                self.position[1] - half_height,
            ];

            // Flip along y-axis. Scale then move.
            graphics::draw(
                ctx,
                &asset_manager.cool_chicken_sprite,
                DrawParam::default()
                    .scale([-1.0, 1.0])
                    .dest(offset_position),
            )?;
        } else {
            let offset_position = [
                self.position[0] - half_width + 10.0, /* Image specific x-offset */
                self.position[1] - half_height,
            ];
            graphics::draw(
                ctx,
                &asset_manager.cool_chicken_sprite,
                DrawParam::default().dest(offset_position),
            )?;
        }

        Ok(())
    }

    fn get_current_state(&self) -> MonsterState {
        self.state
    }
}
