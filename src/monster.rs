use crate::{
    Block,
    BLOCK_SIZE,
    Player,
    asset_manager::AssetManager,
};

use ggez::{
    graphics,
    GameResult,
    Context,
};

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
pub enum MonsterState
// TODO: put in namespace.
{
    Walking,
    Attacking,
    Dead,
}

pub struct Monster {
    pub position: [f32; 2],
    pub speed: f32,
    pub health: f32,
    pub move_goal: usize,
    pub state: MonsterState,
}

impl Monster {
    pub const SIZE: f32 = 20.0;
    pub const DAMAGE: f32 = 1.0;

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
        [
            self.position[0] + Monster::SIZE / 2.0,
            self.position[1] + Monster::SIZE / 2.0,
        ]
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
        let _goal = path_blocks[self.move_goal].pos;
        let goal_x = _goal.0 * BLOCK_SIZE + BLOCK_SIZE / 2.0 - Monster::SIZE / 2.0;
        let goal_y = _goal.1 * BLOCK_SIZE + BLOCK_SIZE / 2.0 - Monster::SIZE / 2.0;
        let goal = (goal_x, goal_y);

        // Distance to next goal position.
        let mut dir = (goal.0 - self.position[0], goal.1 - self.position[1]);
        let mut dist = dir.0 * dir.0 + dir.1 * dir.1;

        // Special case where we are exactly at the right position.
        if dist == 0.0 {
            self.move_goal += 1;
        } else if dist > 0.0 {
            // We have not yet reached the goal destination.

            // Normailze the direction vector so it doesn't scale the speed.
            dist = dist.sqrt();
            dir.0 = dir.0 / dist;
            dir.1 = dir.1 / dist;

            // If 1 step is too far/we pass the goal only move to the goal
            // position/don't overshoot.
            if dist < self.speed * elapsed {
                self.move_goal += 1;

                self.position[0] += dir.0 * dist;
                self.position[1] += dir.1 * dist;
            } else {
                // 1 step will not reach the goal.
                self.position[0] += dir.0 * self.speed * elapsed;
                self.position[1] += dir.1 * self.speed * elapsed;
            }
        }
    }

    pub fn update(&mut self, elapsed: f32, path_blocks: &Vec<Block>, player: &mut Player) {
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

    pub fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult {
        let location = (ggez::mint::Point2 {
            x: self.position[0],
            y: self.position[1] - 10.0,
        },);

        graphics::draw(ctx, &asset_manager.monster_sprite, location)?;

        Ok(())
    }
}
