use std::{f32::consts::PI, time::Duration};

use macroquad::prelude::*;

use super::{RoflcopterAnimation, RoflcopterState};
use crate::helper::*;
use crate::state::State;

#[derive(Debug, Clone)]
pub struct Shot {
    pub position: Vec2,
    pub direction: Side,
    pub angle: f32,
}

impl RoflcopterAnimation {
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
            shot.position += distance;

            // Check enemy collision
            for (enemy_index, enemy) in self.enemies.iter_mut().enumerate() {
                let distance = shot.position - enemy.position;
                if distance.length() < 50.0 {
                    shots_to_remove.push(shot_index);

                    enemy.health -= 1;
                    if enemy.health == 0 {
                        enemies_to_remove.push(enemy_index);
                    }
                }
            }

            // Check if the shot left the image and can be removed
            let text_width = self.textures.shot.width();
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
                    // Don't shoot at enemies, that cannot be seen yet.
                    if outside_screen(state, enemy.position).is_some() {
                        continue;
                    }

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

    pub fn draw_shots(&self) {
        for shot in self.shots.iter() {
            draw_texture_ex(
                self.textures.shot,
                shot.position.x,
                shot.position.y,
                Color::from_rgba(255, 255, 255, 255),
                DrawTextureParams {
                    rotation: shot.angle,
                    flip_y: true,
                    ..Default::default()
                },
            )
        }
    }

    /// Spawn a new shot depending on the current position and copter state.
    pub fn spawn_shot(&mut self, dest: Vec2) {
        let dimensions = self.textures.copter_dimensions();
        // Calculate the middle of the copter.
        let middle = middle_texture_position(self.get_copter_position(), self.textures.texture());

        match self.roflcopter_state {
            RoflcopterState::Flying { .. } => {
                // Check in which direction we're flying.
                let direction = side(&middle, &dest);

                // Rotate the offset depending on the current directoin
                let shot_offset = match direction {
                    Side::Left => {
                        let shot_offset = Vec2::new(-dimensions.x / 2.0, 0.0);
                        rotate_vec2(shot_offset, -PI / 8.0)
                    }
                    Side::Right => {
                        let shot_offset = Vec2::new(dimensions.x / 2.0, 0.0);
                        rotate_vec2(shot_offset, PI / 8.0)
                    }
                };

                let position = middle + shot_offset;
                let distance = dest - position;
                let angle = vec2_to_radian(distance);

                self.shots.push(Shot {
                    position,
                    direction,
                    angle,
                })
            }
            RoflcopterState::Hovering {
                ref copter_direction,
                ..
            } => {
                // Rotate the offset depending on the current directoin
                let shot_offset = match copter_direction {
                    Side::Left => Vec2::new(-dimensions.x / 2.0, 0.0),
                    Side::Right => Vec2::new(dimensions.x / 2.0, 0.0),
                };

                let position = middle + shot_offset;
                let distance = dest - position;
                let angle = vec2_to_radian(distance);

                self.shots.push(Shot {
                    position,
                    direction: copter_direction.clone(),
                    angle,
                })
            }
        }
    }
}
