use std::{f32::consts::PI, time::Duration};

use cgmath::Vector2;
use macroquad::prelude::*;

mod raw_draw;

use self::raw_draw::draw_raw_copter;
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

/// A struct used to store dynamically generated images of the roflcopter.
#[derive(Debug, Clone)]
pub struct CopterImages {
    pub right_copter_right_rotor: Texture2D,
    pub right_copter_left_rotor: Texture2D,
    pub left_copter_right_rotor: Texture2D,
    pub left_copter_left_rotor: Texture2D,
}

impl CopterImages {
    pub fn new(state: &State) -> CopterImages {
        CopterImages {
            right_copter_right_rotor: draw_raw_copter(state, Direction::Right, Direction::Right),
            right_copter_left_rotor: draw_raw_copter(state, Direction::Right, Direction::Left),
            left_copter_right_rotor: draw_raw_copter(state, Direction::Left, Direction::Right),
            left_copter_left_rotor: draw_raw_copter(state, Direction::Left, Direction::Left),
        }
    }

    pub fn update(&mut self, state: &State) {
        self.right_copter_right_rotor = draw_raw_copter(state, Direction::Right, Direction::Right);
        self.right_copter_left_rotor = draw_raw_copter(state, Direction::Right, Direction::Left);
        self.left_copter_right_rotor = draw_raw_copter(state, Direction::Left, Direction::Right);
        self.left_copter_left_rotor = draw_raw_copter(state, Direction::Left, Direction::Left);
    }

    pub fn get_for_directions(
        &self,
        copter_direction: &Direction,
        rotor_direction: &Direction,
    ) -> Texture2D {
        match copter_direction {
            Direction::Right => match rotor_direction {
                Direction::Right => self.right_copter_right_rotor.clone(),
                Direction::Left => self.right_copter_left_rotor.clone(),
            },
            Direction::Left => match rotor_direction {
                Direction::Right => self.left_copter_right_rotor.clone(),
                Direction::Left => self.left_copter_left_rotor.clone(),
            },
        }
    }
}

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
