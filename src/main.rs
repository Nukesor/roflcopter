use macroquad::prelude::*;
use simplelog::{Config, LevelFilter, SimpleLogger};

mod animations;
mod color;
mod shaders;
mod state;

use crate::animations::*;
use crate::state::State;

#[macroquad::main("Text")]
async fn main() {
    setup();
    let mut state = State::new().await;
    state.grab_black_screen();

    let mut animation = Animation::new_wall();
    //Animation::new_copter(&state, Vec2::new(state.window_width / 2.0, state.window_height / 2.0));

    loop {
        println!("FPS: {}", get_fps());
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
        }

        if let Some(next_animation) = state.update(&mut animation) {
            animation = next_animation;
        }

        if let Some(ref transition) = state.transition {
            // Calculate the gradiant, depending on the current state of the transition and,
            // whether it's a phase in or a phase out.
            let gradiant = (transition.timer.as_millis() as f64
                / state.transition_duration.as_millis() as f64) as f32;
            let gradiant = match transition.phase {
                state::Phase::In => 1.0 - gradiant,
                state::Phase::Out => gradiant,
            };

            draw_texture_ex(
                state.black_screen,
                0.0,
                0.0,
                Color::new(0.0, 0.0, 0.0, gradiant),
                DrawTextureParams {
                    flip_y: true,
                    ..Default::default()
                },
            )
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
