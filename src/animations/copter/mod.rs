use std::{
    f32::consts::PI,
    ops::{Add, Sub},
    time::Duration,
};

use macroquad::{prelude::*, rand::gen_range};

mod draw;
mod images;

use self::draw::{draw_copter, draw_shot};
pub use self::images::CopterImages;
use crate::animations::helper::{delta_duration, Direction};
use crate::state::State;

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
pub struct Shot {
    position: Vec2,
    direction: Direction,
    angle: f32,
}

#[derive(Debug, Clone)]
pub struct CopterAnimation {
    pub rotor_direction: Direction,
    pub rotor_duration: Duration,
    pub rotor_timer: Duration,

    pub copter_images: CopterImages,
    pub copter_state: CopterState,

    pub shot_timeout: Duration,
    pub shot_timer: Duration,
    shots: Vec<Shot>,
}

impl CopterAnimation {
    pub fn new(state: &State, position: Vec2) -> CopterAnimation {
        let copter_state = CopterState::Flying {
            position,
            dest: Vec2::new(100.0, 100.0),
        };
        CopterAnimation {
            rotor_direction: Direction::Left,
            rotor_duration: Duration::from_millis(200),
            rotor_timer: Duration::from_secs(0),

            copter_images: CopterImages::new(state),
            copter_state,

            shot_timeout: Duration::from_millis(150),
            shot_timer: Duration::from_secs(0),
            shots: vec![],
        }
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

        self.handle_shots(state);

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
        for shot in self.shots.iter() {
            draw_shot(&self.copter_images, shot);
        }

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

    /// Tick all shots and spawn new ones, if the mouse is down.
    pub fn handle_shots(&mut self, state: &State) {
        // Animate the shots
        let mut to_remove = vec![];
        for (index, shot) in self.shots.iter_mut().enumerate() {
            let speed = state.window_width / 60.0;

            let direction = Vec2::new(shot.angle.cos(), shot.angle.sin());
            let distance = direction * speed;

            shot.position = shot.position + distance;

            let text_width = self.copter_images.shot.width();
            if shot.position.x > state.window_width
                || shot.position.x < 0.0 - text_width
                || shot.position.y > state.window_height + text_width
                || shot.position.y < 0.0 - text_width
            {
                to_remove.push(index);
            }
        }
        to_remove.reverse();
        for index in to_remove {
            self.shots.remove(index);
        }

        // Check if we want to spawn new shots.
        if is_mouse_button_down(MouseButton::Left) {
            if self.shot_timer.as_micros() == 0 {
                self.fire_shot(Vec2::new(state.mouse_position.0, state.mouse_position.1))
            }

            self.shot_timer = self.shot_timer.checked_add(delta_duration()).unwrap();
            if self.shot_timer > self.shot_timeout {
                self.shot_timer = Duration::from_secs(0);
            }
        }
    }

    /// Spawn a new shot depending on the current position and copter state.
    pub fn fire_shot(&mut self, dest: Vec2) {
        match self.copter_state {
            CopterState::Flying { ref position, .. } => {
                let direction = if position.x > dest.x {
                    Direction::Left
                } else {
                    Direction::Right
                };

                let distance = dest - position.clone();
                let angle = distance.y.atan2(distance.x);
                self.shots.push(Shot {
                    position: position.clone(),
                    direction,
                    angle,
                })
            }
            CopterState::Hovering {
                ref position,
                ref copter_direction,
                ..
            } => {
                let angle = position.angle_between(dest);
                self.shots.push(Shot {
                    position: position.clone(),
                    direction: copter_direction.clone(),
                    angle,
                })
            }
        }
    }
}
