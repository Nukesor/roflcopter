use std::{
    f32::consts::PI,
    ops::{Add, Sub},
    time::Duration,
};

use macroquad::{prelude::*, rand::gen_range};

use super::{draw::draw_copter, CopterImages};
use crate::{
    animations::{
        helper::{delta_duration, Direction},
        Animation,
    },
    state::State,
};

#[derive(Debug, Clone)]
pub enum CopterState {
    Flying {
        position: Vec2,
        dest: Vec2,
    },
    Hovering {
        duration: Duration,
        timer: Duration,
        position: Vec2,
        copter_direction: Direction,
    },
}

#[derive(Debug, Clone)]
pub struct CopterAnimation {
    pub rotor_direction: Direction,
    pub rotor_duration: Duration,
    pub rotor_timer: Duration,
    pub copter_images: CopterImages,
    pub copter_state: CopterState,
}

impl CopterAnimation {
    pub fn new(state: &State, position: Vec2) -> Animation {
        Animation::Copter(CopterAnimation {
            rotor_direction: Direction::Left,
            rotor_duration: Duration::from_millis(200),
            rotor_timer: Duration::from_secs(0),
            copter_images: CopterImages::new(state),
            copter_state: CopterState::Flying {
                position,
                dest: Vec2::new(100.0, 100.0),
            },
        })
    }

    pub fn update(&mut self, state: &State) {
        // This is the rotor animation.
        // This animation is always active.
        self.rotor_timer = self.rotor_timer.checked_add(delta_duration()).unwrap();
        if self.rotor_timer > self.rotor_duration {
            match self.rotor_direction {
                Direction::Left => self.rotor_direction = Direction::Right,
                Direction::Right => self.rotor_direction = Direction::Left,
            }
            self.rotor_timer = Duration::from_secs(0);
        }

        let mut next_state: Option<CopterState> = None;
        // Update state dependant variables.
        match self.copter_state {
            CopterState::Flying {
                ref mut position,
                ref dest,
            } => {
                // Check, whether we reached our position
                if dest.sub(position.clone()).length() < 5.0 {
                    let copter_direction = if position.x > dest.x {
                        Direction::Left
                    } else {
                        Direction::Right
                    };

                    next_state = Some(CopterState::Hovering {
                        duration: Duration::from_secs(gen_range(1, 3)),
                        timer: Duration::from_secs(0),
                        position: position.clone(),
                        copter_direction,
                    })
                } else {
                    // The speed per second is relative to the screen width.
                    let speed = state.window_width / 4.0;

                    // Calculate the traveled distance for this frame
                    let direction = dest.sub(position.clone());
                    let normalized = direction.normalize();
                    let traveling = normalized * speed * get_frame_time();

                    *position = position.add(traveling);
                }
            }
            CopterState::Hovering {
                ref mut timer,
                ref duration,
                ref position,
                ..
            } => {
                // We're done with hovering, pick a random position on the screen.
                // We only pick positions, where the copter can be fully seen.
                if *timer > *duration {
                    let height = state.window_height;
                    let width = state.window_width;

                    let image_height = self.copter_images.right_copter_right_rotor.height();
                    let image_width = self.copter_images.right_copter_right_rotor.width();

                    next_state = Some(CopterState::Flying {
                        position: position.clone(),
                        dest: Vec2::new(
                            gen_range(0.0, width - image_width),
                            gen_range(0.0, height - image_height),
                        ),
                    });
                }
                *timer = timer.checked_add(delta_duration()).unwrap();
            }
        }

        if let Some(next_state) = next_state {
            self.copter_state = next_state;
        }
    }

    /// Draw the copter depending on the current animation state.
    pub fn draw(&mut self, state: &State) {
        match self.copter_state {
            CopterState::Flying {
                ref mut position,
                ref dest,
            } => {
                let copter_direction = if position.x > dest.x {
                    Direction::Left
                } else {
                    Direction::Right
                };

                let angle = match copter_direction {
                    Direction::Left => -PI as f32 / 8.0,
                    Direction::Right => PI as f32 / 8.0,
                };

                draw_copter(
                    &self.copter_images,
                    &copter_direction,
                    &self.rotor_direction,
                    position.x,
                    position.y,
                    angle,
                );
            }
            CopterState::Hovering {
                ref duration,
                ref timer,
                ref position,
                ref copter_direction,
            } => {
                // We animate hovering by following in a sinus curve depending on the time
                let mut current_rotation = timer.as_millis() as f32 / duration.as_millis() as f32;
                // Speed up things a little
                current_rotation = current_rotation * 1.4;
                let offset = (current_rotation * 2.0 * PI).sin();
                let x = position.x;
                let y = position.y + offset * state.font_dimensions.height;

                draw_copter(
                    &self.copter_images,
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
