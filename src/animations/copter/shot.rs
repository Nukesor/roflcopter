use std::{f32::consts::PI, time::Duration};

use macroquad::prelude::*;

use super::{CopterAnimation, CopterState};
use crate::helper::*;
use crate::state::State;

#[derive(Debug, Clone)]
pub struct Shot {
    pub position: Vec2,
    pub direction: Side,
    pub angle: f32,
}

impl CopterAnimation {
    /// Tick all shots and spawn new ones, if the mouse is down.
    pub fn update_shots(&mut self, state: &State) {
        // Move all shots and check if they hit something or are off screen.
        let mut shots_to_remove = Vec::new();
        let mut enemies_to_remove = Vec::new();
        for (shot_index, shot) in self.shots.iter_mut().enumerate() {
            // Update the shot's positoin
            let speed = state.window_width / 60.0;
            let direction = Vec2::new(shot.angle.cos(), shot.angle.sin());
            let distance = direction * speed;
            shot.position = shot.position + distance;

            // Check enemy collision
            for (enemy_index, enemy) in self.enemies.iter().enumerate() {
                let distance = shot.position - enemy.position;
                if distance.length() < 50.0 {
                    enemies_to_remove.push(enemy_index);
                    shots_to_remove.push(shot_index);
                }
            }

            // Check if the shot left the image and can be removed
            let text_width = self.copter_images.shot.width();
            if shot.position.x > state.window_width
                || shot.position.x < 0.0 - text_width
                || shot.position.y > state.window_height + text_width
                || shot.position.y < 0.0 - text_width
            {
                shots_to_remove.push(shot_index);
            }
        }

        shots_to_remove.sort();
        shots_to_remove.dedup();
        shots_to_remove.reverse();
        for index in shots_to_remove {
            self.shots.remove(index);
        }

        enemies_to_remove.sort();
        enemies_to_remove.dedup();
        enemies_to_remove.reverse();
        for index in enemies_to_remove {
            self.enemies.remove(index);
        }

        // Check if we want to spawn new shots.
        if self.shot_timer.as_micros() == 0 {
            let copter_position = self.get_copter_position();
            if is_mouse_button_down(MouseButton::Left) {
                self.spawn_shot(Vec2::new(state.mouse_position.0, state.mouse_position.1))
            } else {
                // Get the closest enemy.
                let mut best_position: Option<(f32, Vec2)> = None;
                for enemy in self.enemies.iter() {
                    let distance = (copter_position - enemy.position).length();
                    if let Some((old_distance, _)) = best_position {
                        if old_distance > distance {
                            best_position = Some((distance, enemy.position));
                        }
                    } else {
                        best_position = Some((distance, enemy.position));
                    }
                }

                // Fire a shot, if we found an enemy.
                if let Some((_, position)) = best_position {
                    self.spawn_shot(position);
                }
            }
        }

        // Tick the shot timer
        self.shot_timer = self.shot_timer.checked_add(delta_duration()).unwrap();
        if self.shot_timer > self.shot_timeout {
            self.shot_timer = Duration::from_secs(0);
        }
    }

    /// Spawn a new shot depending on the current position and copter state.
    pub fn spawn_shot(&mut self, dest: Vec2) {
        // Calculate the middle of the copter.
        let dimensions = self.copter_images.copter_dimensions();
        let copter_position = self.get_copter_position();
        let middle = copter_position + Vec2::new(dimensions.0 / 2.0, dimensions.1 / 2.0);

        match self.copter_state {
            CopterState::Flying { .. } => {
                // Check in which direction we're flying.
                let direction = side(&middle, &dest);

                // Rotate the offset depending on the current directoin
                let shot_offset = match direction {
                    Side::Left => {
                        let shot_offset = Vec2::new(-dimensions.0 / 2.0, 0.0);
                        rotate_vec2(shot_offset, -PI / 8.0)
                    }
                    Side::Right => {
                        let shot_offset = Vec2::new(dimensions.0 / 2.0, 0.0);
                        rotate_vec2(shot_offset, PI / 8.0)
                    }
                };

                let position = middle + shot_offset;
                let distance = dest - position;
                let angle = distance.y.atan2(distance.x);

                self.shots.push(Shot {
                    position,
                    direction,
                    angle,
                })
            }
            CopterState::Hovering {
                ref copter_direction,
                ..
            } => {
                // Rotate the offset depending on the current directoin
                let shot_offset = match copter_direction {
                    Side::Left => Vec2::new(-dimensions.0 / 2.0, 0.0),
                    Side::Right => Vec2::new(dimensions.0 / 2.0, 0.0),
                };

                let position = middle + shot_offset;
                let distance = dest - position;
                let angle = distance.y.atan2(distance.x);

                self.shots.push(Shot {
                    position: position.clone(),
                    direction: copter_direction.clone(),
                    angle,
                })
            }
        }
    }
}
