use cgmath::Vector2;
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

    let window_height = screen_height();
    let window_width = screen_width();

    let mut _animation = Animation::Wall(WallAnimation {
        y_offset: 0.0,
        x_offset: 0.0,
    });
    let mut animation = CopterAnimation::new(Vector2::new(window_width / 2.0, window_height / 2.0));

    loop {
        clear_background(BLACK);

        match animation {
            Animation::Wall(ref mut inner) => {
                animate_wall(&state, inner);
            }
            Animation::Copter(ref mut inner) => {
                animate_copter(&state, inner);
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
