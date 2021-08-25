use macroquad::prelude::*;
use simplelog::{Config, LevelFilter, SimpleLogger};

mod animations;
mod color;
mod state;

use crate::animations::*;
use crate::state::State;

#[macroquad::main("Text")]
async fn main() {
    setup();

    let state = State::new().await;
    let mut animation = Animation::Wall(WallAnimation {
        y_offset: 0.0,
        x_offset: 0.0,
    });

    loop {
        clear_background(BLACK);

        match animation {
            Animation::Wall(ref mut inner) => {
                animate_wall(&state, inner);
            }
        }

        next_frame().await
    }
}

fn setup() {
    let verbosity = 0;
    // Set the verbosity level of the logger.
    let level = match verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };
    SimpleLogger::init(level, Config::default()).unwrap();
}
