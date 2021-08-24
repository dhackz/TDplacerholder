use crate::{
    asset_system::AssetManager, game_components::GoldPile, game_views::monsters::MonsterView,
};

use ggez::{Context, GameResult};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TowerType {
    Basic,
    Ninja,
}

pub trait Tower {
    fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult;

    fn draw_abilities(
        &mut self,
        ctx: &mut Context,
        monster_views: &Vec<Box<dyn MonsterView>>,
    ) -> GameResult;

    fn update(
        &mut self,
        elapsed: f32,
        monster_views: &mut Vec<Box<dyn MonsterView>>, //TODO: workaround to make separating monster component/view easier.
        gold_piles: &mut Vec<GoldPile>,
        asset_manager: &mut AssetManager,
    );

    fn get_block_position(&self) -> [f32; 2];
}
