use std::f32::consts::PI;

use macroquad::{prelude::Vec2, rand::gen_range};

use super::structures::*;
use crate::state::State;

pub fn side(src: &Vec2, dest: &Vec2) -> Side {
    if src.x > dest.x {
        Side::Left
    } else {
        Side::Right
    }
}

/// Return the angle of a vector as radian.
pub fn vec2_to_radian(vec: Vec2) -> f32 {
    vec.y.atan2(vec.x)
}

/// Rotate a vector clock-wise around an angle in radians.
pub fn rotate_vec2(vec: Vec2, angle: f32) -> Vec2 {
    let x = vec.x * angle.cos() - vec.y * angle.sin();
    let y = vec.x * angle.sin() + vec.y * angle.cos();

    Vec2::new(x, y)
}

/// Create a new vector with a random direction and a given length.
pub fn random_vector_with_lenght(length: f32) -> Vec2 {
    let vec = Vec2::new(length, 0.0);
    rotate_vec2(vec, gen_range(0.0, PI * 2.0))
}

pub fn outside_screen(state: &State, position: Vec2) -> Option<Direction> {
    // Check collisions on all sides
    if position.x >= state.window_width {
        return Some(Direction::Right);
    } else if position.x <= -0.0 {
        return Some(Direction::Left);
    } else if position.y >= state.window_height {
        return Some(Direction::Bottom);
    } else if position.y <= -0.0 {
        return Some(Direction::Top);
    }

    None
}
