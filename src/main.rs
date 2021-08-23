extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::main_state::MainState;

use ggez::{
    conf::{FullscreenType, WindowMode},
    event, GameResult,
};

use std::env;
use std::path;

mod animation_system;
mod asset_system;
mod game_components;
mod level_system;
mod main_state;
mod ui_system;
mod utils;

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
        width: 800.0,
        height: 600.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: true,
    };

    let cb = ggez::ContextBuilder::new("TowerOfDerp", "rrEd")
        .add_resource_path(resource_dir)
        .window_mode(mode);
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx);
    event::run(ctx, event_loop, state)
}
