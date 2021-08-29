use std::time::Duration;

use macroquad::{
    prelude::*,
    rand::{gen_range, ChooseRandom},
};

use crate::state::State;

pub mod structures;
pub mod texture;
pub mod vector;

pub use structures::*;
pub use texture::*;
pub use vector::*;

/// Helper, which returns macroquad's current delta frame time as std Duration.
pub fn delta_duration() -> Duration {
    let dt = (get_frame_time() * 1000.0 * 1000.0) as u64;
    Duration::from_micros(dt)
}

/// Return a random color.
pub fn random_color() -> Color {
    Color::from_rgba(gen_range(0, 255), gen_range(0, 255), gen_range(0, 255), 255)
}

pub fn random_position_on_screen(state: &State) -> Vec2 {
    Vec2::new(
        gen_range(100.0, state.window_width - 100.0),
        gen_range(100.0, state.window_height - 100.0),
    )
}

/// Return a random position at the edges of the screen.
/// The position can be offset by a given amount.
pub fn random_position_outside_screen(state: &State, offset: f32) -> Vec2 {
    let mut position = Vec2::new(
        gen_range(0.0, state.window_width),
        gen_range(0.0, state.window_height),
    );

    let directions = vec![
        Direction::Top,
        Direction::Bottom,
        Direction::Left,
        Direction::Right,
    ];
    let direction = directions.choose().expect("Failed to get random direction");

    match direction {
        Direction::Top => position.y = -offset,
        Direction::Bottom => position.y = state.window_height + offset,
        Direction::Left => position.x = -offset,
        Direction::Right => position.x = state.window_width + offset,
    }

    position
}
