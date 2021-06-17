extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::{
    block::{Block, BLOCK_SIZE},
    board::Board,
    main_state::MainState,
    player::Player,
};

use ggez::{event, GameResult};

use std::env;
use std::path;

mod asset_manager;
mod base;
mod block;
mod board;
mod gold;
mod main_state;
mod monster;
mod monster_spawner;
mod player;
mod towers;
mod ui;

fn main() -> GameResult {
    pretty_env_logger::init();

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
