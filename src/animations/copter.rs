use std::{f32::consts::PI, ops::Add, time::Duration};

use macroquad::prelude::*;

use super::{helper::delta_duration, CopterAnimation, Direction, Position};
use crate::state::State;

#[derive(Debug, Clone)]
pub enum CopterState {
    Flying {
        position: Position,
        dest: Position,
    },
    Hovering {
        duration: Duration,
        timer: Duration,
        position: Position,
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

fn draw(state: &State, animation: &mut CopterAnimation) {
    match animation.state {
        CopterState::Flying {
            ref position,
            ref dest,
        } => {
            draw_copter(state, &animation.rotor_direction, position.x, position.y);
        }
        CopterState::Hovering {
            ref duration,
            ref mut timer,
            ref position,
        } => {
            // We animate hovering by following in a sinus curve depending on the time
            let current_rotation = timer.as_millis() as f32 / duration.as_millis() as f32;
            let offset = (current_rotation * 2.0 * PI).sin();
            let x_start = position.x;
            let y_start = position.y + offset * state.font_dimensions.height;

            *timer = timer.checked_add(delta_duration()).unwrap();
            draw_copter(state, &animation.rotor_direction, x_start, y_start);
        }
    }
}

fn draw_copter(state: &State, rotor_direction: &Direction, x_start: f32, mut y: f32) {
    let mut x: f32;
    let art = get_ascii_art(rotor_direction);

    //println!("timer: {:?}, duration: {:?}", timer, duration);
    //println!("offset: {}", offset);
    //println!("current_rotation: {}", offset);
    for line in art.lines() {
        x = x_start;
        for character in line.chars() {
            draw_text_ex(
                &character.to_string(),
                x,
                y,
                TextParams {
                    font: state.font,
                    font_size: state.font_size,
                    font_scale: 1.0,
                    ..Default::default()
                },
            );
            x += state.font_dimensions.width;
        }

        // Move the draw position to the next
        y += state.font_dimensions.height;
    }
}

fn get_ascii_art(direction: &Direction) -> String {
    match direction {
        Direction::Left => "
   ROFL:ROFL:
         ___^___ _
 L    __/      [] \\
LOL===__           \\
 L      \\___ ___ ___]
            I   I
          ----------/"
            .to_string(),
        Direction::Right => "
            :ROFL:ROFL
         ___^___ _
L L   __/      [] \\
 O ===__           \\
L L     \\___ ___ ___]
            I   I
          ----------/"
            .to_string(),
    }
}
