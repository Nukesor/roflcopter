use std::{f32::consts::PI, ops::Add, time::Duration};

use macroquad::prelude::*;

use super::{CopterAnimation, Direction, Position};
use crate::state::State;

#[derive(Debug, Clone)]
pub enum CopterState {
    Flying {
        src: Position,
        dest: Position,
    },
    Hovering {
        duration: Duration,
        timer: Duration,
        position: Position,
    },
}

pub fn animate_copter(state: &State, animation: &mut CopterAnimation) {
    //let window_height = screen_height();
    //let window_width = screen_width();

    draw(state, animation);
}

fn draw(state: &State, animation: &mut CopterAnimation) {
    match animation.state {
        CopterState::Flying { .. } => {}
        CopterState::Hovering {
            ref duration,
            ref mut timer,
            ref position,
        } => {
            let dt = (get_frame_time() * 1000.0 * 1000.0) as u64;
            *timer = timer.checked_add(Duration::from_micros(dt)).unwrap();
            draw_hover(state, &animation.rotor_direction, duration, timer, position);
        }
    }
}

fn draw_hover(
    state: &State,
    rotor_direction: &Direction,
    duration: &Duration,
    timer: &mut Duration,
    position: &Position,
) {
    let art = get_ascii_art(rotor_direction);

    //println!("timer: {:?}, duration: {:?}", timer, duration);
    //println!("offset: {}", offset);
    //println!("current_rotation: {}", offset);

    // We animate hovering by following in a sinus curve depending on the time
    let current_rotation = timer.as_millis() as f32 / duration.as_millis() as f32;
    let offset = (current_rotation * 2.0 * PI).sin();
    let x_start_position = position.x;
    let mut x_position = x_start_position;
    let mut y_position = position.y + offset * state.font_dimensions.height;

    for line in art.lines() {
        x_position = x_start_position;
        for character in line.chars() {
            draw_text_ex(
                &character.to_string(),
                x_position,
                y_position,
                TextParams {
                    font: state.font,
                    font_size: state.font_size,
                    font_scale: 1.0,
                    ..Default::default()
                },
            );
            x_position += state.font_dimensions.width;
        }

        // Move the draw position to the next
        y_position += state.font_dimensions.height;
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
