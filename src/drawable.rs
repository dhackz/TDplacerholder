use crate::{
    asset_manager::AssetManager,
    monster::Monster,
    gold::GoldPile,
};

use ggez::{
    graphics,
    GameResult,
    Context,
};

pub trait Tower {
    fn draw(
        &mut self,
        ctx: &mut Context,
        asset_manager: &AssetManager
    ) -> GameResult;

    fn update(
        &mut self,
        elapsed: f32,
        monsters: &mut Vec<Monster>,
        gold_piles: &mut Vec<GoldPile>,
        asset_manager: &mut AssetManager,
    );
}
