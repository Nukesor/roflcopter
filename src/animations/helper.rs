use std::time::Duration;

use macroquad::prelude::{get_frame_time, Vec2};

#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
}

pub fn delta_duration() -> Duration {
    let dt = (get_frame_time() * 1000.0 * 1000.0) as u64;
    Duration::from_micros(dt)
}

pub fn direction(src: &Vec2, dest: &Vec2) -> Direction {
    if src.x > dest.x {
        Direction::Left
    } else {
        Direction::Right
    }
}
