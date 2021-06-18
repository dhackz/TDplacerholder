use crate::{asset_manager::AssetManager, gold::GoldPile, monsters::monster::Monster};

use ggez::{Context, GameResult};

pub trait Tower {
    fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult;

    fn draw_abilities(&mut self, ctx: &mut Context, monsters: &Vec<Box<dyn Monster>>)
        -> GameResult;

    fn update(
        &mut self,
        elapsed: f32,
        monsters: &mut Vec<Box<dyn Monster>>,
        gold_piles: &mut Vec<GoldPile>,
        asset_manager: &mut AssetManager,
    );
}
