use crate::asset_system::AssetManager;
use crate::game_components::monsters::Monster;

use ggez::{Context, GameResult};

pub trait MonsterView {
    fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult;
    fn get_monster_mut(&mut self) -> &mut dyn Monster;
    fn get_monster(&self) -> &dyn Monster;
}
