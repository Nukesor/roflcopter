use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;
use simplelog::{Config, LevelFilter, SimpleLogger};

use roflcopter_lib::animations::*;
use roflcopter_lib::state::State;

fn window_conf() -> Conf {
    Conf {
        window_title: "Roflcopter".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Set seed for randomness.
    let current_millisecond = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Unable to read systemt time.");
    rand::srand(current_millisecond.as_secs());

    setup();
    let mut state = State::new().await;
    state.grab_black_screen();

    //let mut animation = Animation::new_wall();
    //let mut animation = Animation::new_copter(
    //    &state,
    //    Vec2::new(state.window_width / 2.0, state.window_height / 2.0),
    //);
    let mut animation = Animation::new_word_chaos(&state);

    loop {
        clear_background(BLACK);

        // We're cycling through animations, only one can run at a time.
        match animation {
            Animation::Wall(ref mut inner) => {
                inner.update(&state);
                inner.draw(&state);
            }
            Animation::Copter(ref mut inner) => {
                inner.update(&state);
                inner.draw(&state);
            }
            Animation::WordChaos(ref mut inner) => {
                inner.update(&state);
                inner.draw(&state);
            }
            Animation::Snake(ref mut inner) => {
                inner.update();
                inner.draw();
            }
        }

        state.draw();

        if let Some(next_animation) = state.update(&mut animation) {
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
