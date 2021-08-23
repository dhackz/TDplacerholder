use crate::{
    asset_system::AssetManager,
    game_components::{Block, GoldPile, Player},
};

use ggez::{Context, GameResult};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum MonsterType {
    Chicken,
    CoolChicken,
}

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Debug)]
pub enum MonsterState {
    Walking,
    Attacking,
    Dead,
}

pub trait Monster {
    fn get_center_pos_abs(&self) -> [f32; 2];

    fn recieve_damage(
        &mut self,
        damage: f32,
        gold_piles: &mut Vec<GoldPile>,
        asset_manager: &mut AssetManager,
    );

    fn get_current_state(&self) -> MonsterState;

    fn update(&mut self, elapsed: f32, path_blocks: &Vec<Block>, player: &mut Player);

    fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult;
}
