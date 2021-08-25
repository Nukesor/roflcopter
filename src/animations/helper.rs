use std::time::Duration;

use macroquad::prelude::get_frame_time;

#[derive(Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
}

pub fn delta_duration() -> Duration {
    let dt = (get_frame_time() * 1000.0 * 1000.0) as u64;
    Duration::from_micros(dt)
}
