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

    let mut state = State::new().await;

    let window_height = screen_height();
    let window_width = screen_width();

    let mut animation =
        CopterAnimation::new(&state, Vec2::new(window_width / 2.0, window_height / 2.0));

    loop {
        clear_background(BLACK);

        // We're cycling through animations, only one can run at a time.
        match animation {
            Animation::Wall(ref mut inner) => {
                animate_wall(&state, inner);
            }
            Animation::Copter(ref mut inner) => {
                animate_copter(&state, inner);
            }
        }

        if let Some(next_animation) = state.update(&animation) {
            animation = next_animation;
        }

        next_frame().await
    }
}

fn setup() {
    // Beautify panics for better debug output.
    better_panic::install();

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
