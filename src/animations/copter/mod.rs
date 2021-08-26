use std::{f32::consts::PI, time::Duration};

use cgmath::Vector2;
use macroquad::prelude::*;

mod images;
mod raw_draw;

pub use self::images::CopterImages;
use super::{helper::delta_duration, CopterAnimation, Direction};
use crate::state::State;

#[derive(Debug, Clone)]
pub enum CopterState {
    Flying {
        position: Vector2<f32>,
        dest: Vector2<f32>,
    },
    Hovering {
        duration: Duration,
        timer: Duration,
        position: Vector2<f32>,
        copter_direction: Direction,
    },
}

pub fn animate_copter(state: &State, animation: &mut CopterAnimation) {
    animation.rotor_timer = animation.rotor_timer.checked_add(delta_duration()).unwrap();
    if animation.rotor_timer > animation.rotor_duration {
        match animation.rotor_direction {
            Direction::Left => animation.rotor_direction = Direction::Right,
            Direction::Right => animation.rotor_direction = Direction::Left,
        }
        animation.rotor_timer = Duration::from_secs(0)
    }
    draw(state, animation);
}

/// Draw the copter depending on the current animation state.
fn draw(state: &State, animation: &mut CopterAnimation) {
    match animation.state {
        CopterState::Flying {
            ref position,
            ref dest,
        } => {
            let copter_direction = if position.x < dest.x {
                Direction::Left
            } else {
                Direction::Right
            };

            draw_copter(
                &animation.copter_images,
                position.x,
                position.y,
                &copter_direction,
                &animation.rotor_direction,
            );
        }
        CopterState::Hovering {
            ref duration,
            ref mut timer,
            ref position,
            ref copter_direction,
        } => {
            // We animate hovering by following in a sinus curve depending on the time
            let current_rotation = timer.as_millis() as f32 / duration.as_millis() as f32;
            let offset = (current_rotation * 2.0 * PI).sin();
            let x = position.x;
            let y = position.y + offset * state.font_dimensions.height;

            *timer = timer.checked_add(delta_duration()).unwrap();
            draw_copter(
                &animation.copter_images,
                x,
                y,
                copter_direction,
                &animation.rotor_direction,
            );
        }
    }
}

/// Lower level helicopter drawing call.
/// This is a simple wrapper around some of macroquads drawing logic.
fn draw_copter(
    images: &CopterImages,
    x: f32,
    y: f32,
    copter_direction: &Direction,
    rotor_direction: &Direction,
) {
    let texture = images.get_for_directions(copter_direction, rotor_direction);
    draw_texture_ex(
        texture,
        x,
        y,
        Color::from_rgba(255, 255, 255, 255),
        DrawTextureParams {
            rotation: -PI as f32 / 8.0,
            flip_y: true,
            ..Default::default()
        },
    )
}
