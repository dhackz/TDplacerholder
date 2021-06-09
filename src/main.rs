use crate::{
    block::{ Block,  BLOCK_SIZE, },
    player::Player,
    board::Board,
    main_state::MainState,
};

use ggez::{ GameResult, event, };

use std::env;
use std::path;

mod asset_manager;
mod base;
mod block;
mod player;
mod ui;
mod monster;
mod tower;
mod monster_spawner;
mod board;
mod main_state;


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
