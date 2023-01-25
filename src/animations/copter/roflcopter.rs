use std::{
    f32::consts::PI,
    ops::{Add, Sub},
    time::Duration,
};

use macroquad::{prelude::*, rand::gen_range};

use super::{draw::draw_roflcopter, RoflcopterAnimation, RoflcopterState};
use crate::helper::*;
use crate::state::State;

impl RoflcopterAnimation {
    /// Move the helicopter and tick any timers.
    ///
    /// This logic is also responsible for setting the next internal state once the previous
    /// animation has finished.
    pub fn update_roflcopter(&mut self, state: &State) {
        // This is the rotor animation.
        // This animation is always active.
        self.rotor_timer = self.rotor_timer.checked_add(delta_duration()).unwrap();
        if self.rotor_timer > self.rotor_duration {
            match self.rotor_direction {
                Side::Left => self.rotor_direction = Side::Right,
                Side::Right => self.rotor_direction = Side::Left,
            }
            self.rotor_timer = Duration::from_secs(0);
        }

        // Update state dependant variables.
        match self.roflcopter_state {
            RoflcopterState::Flying {
                ref mut position,
                ref dest,
            } => {
                // Update the helicopter's position.
                // The speed per second is relative to the screen width.
                let speed = state.window_width / 4.0;
                // Calculate the traveled distance for this frame
                let direction = dest.sub(*position);
                let normalized = direction.normalize();
                let traveling = normalized * speed * get_frame_time();

                *position = position.add(traveling);

                // If we reached the position, start hovering for a random time.
                if dest.sub(*position).length() < 50.0 {
                    let copter_direction = if position.x > dest.x {
                        Side::Left
                    } else {
                        Side::Right
                    };

                    self.roflcopter_state = RoflcopterState::Hovering {
                        duration: Duration::from_secs(gen_range(1, 3)),
                        timer: Duration::from_secs(0),
                        position: *position,
                        copter_direction,
                    };
                }
            }
            RoflcopterState::Hovering {
                ref mut timer,
                ref duration,
                ref position,
                ..
            } => {
                *timer = timer.checked_add(delta_duration()).unwrap();
                // Wait until we're done hovering.
                if *timer < *duration {
                    return;
                }

                // We're done hovering, pick a random position on the screen.
                // We only pick positions, where the copter can be fully seen.
                let height = state.window_height;
                let width = state.window_width;

                let dimensions = self.textures.copter_dimensions();

                self.roflcopter_state = RoflcopterState::Flying {
                    position: *position,
                    dest: Vec2::new(
                        gen_range(0.0, width - dimensions.x),
                        gen_range(0.0, height - dimensions.y),
                    ),
                };
            }
        }
    }

    /// Draw the helicopter, depending on the current state.
    pub fn draw_roflcopter(&self, state: &State) {
        match self.roflcopter_state {
            RoflcopterState::Flying {
                ref position,
                ref dest,
            } => {
                let copter_direction = side(position, dest);

                let angle = match copter_direction {
                    Side::Left => -PI / 8.0,
                    Side::Right => PI / 8.0,
                };

                draw_roflcopter(
                    &self.textures,
                    &copter_direction,
                    &self.rotor_direction,
                    position.x,
                    position.y,
                    angle,
                );
            }
            RoflcopterState::Hovering {
                ref timer,
                ref position,
                ref copter_direction,
                ..
            } => {
                // We animate hovering by following in a sinus curve depending on the time
                let mut current_rotation = (timer.as_millis() as f64 / 1000.0f64) as f32;
                // Speed up things a little
                current_rotation *= 1.2;

                let offset = (current_rotation * 2.0 * PI).sin();
                let x = position.x;
                let y = position.y + offset * state.font_dimensions.height;

                draw_roflcopter(
                    &self.textures,
                    copter_direction,
                    &self.rotor_direction,
                    x,
                    y,
                    0.0,
                );
            }
        }
    }
}
