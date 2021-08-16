extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::{
    block::{Block, BLOCK_SIZE},
    board::Board,
    main_state::MainState,
    player::Player,
};

use ggez::{
    conf::{FullscreenType, WindowMode},
    event, GameResult,
};

use std::env;
use std::path;

mod asset_manager;
mod base;
mod block;
mod board;
mod gold;
mod main_state;
mod monster_spawner;
mod player;
mod tower_icon;
mod ui;
mod utils;

mod monsters;
mod towers;

fn main() -> GameResult {
    pretty_env_logger::init();

    let resource_dir = if let Ok(manifest_file) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_file);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let mode = WindowMode {
        width: 300.0,
        height: 600.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };

    let cb = ggez::ContextBuilder::new("TowerOfDerp", "rrEd")
        .add_resource_path(resource_dir)
        .window_mode(mode);
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx);
    event::run(ctx, event_loop, state)
}
