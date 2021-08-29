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
use crate::helper::*;
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
        copter_direction: Side,
    },
}

#[derive(Debug, Clone)]
pub struct Shot {
    position: Vec2,
    direction: Side,
    angle: f32,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    position: Vec2,
    health: usize,
}

impl Enemy {
    pub fn new(position: Vec2) -> Self {
        Enemy {
            position,
            health: 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CopterAnimation {
    rotor_direction: Side,
    rotor_duration: Duration,
    rotor_timer: Duration,

    pub copter_images: CopterImages,
    pub copter_state: CopterState,

    shot_timeout: Duration,
    shot_timer: Duration,
    shots: Vec<Shot>,

    enemies: Vec<Enemy>,
    spawn_enemies: bool,
    enemy_speed: f32,
    enemy_texture: Texture2D,
    enemy_timer: Duration,
    enemy_duration: Duration,
}

impl CopterAnimation {
    pub fn new(state: &State, position: Vec2) -> CopterAnimation {
        let copter_state = CopterState::Flying {
            position,
            dest: Vec2::new(100.0, 100.0),
        };

        CopterAnimation {
            rotor_direction: Side::Left,
            rotor_duration: Duration::from_millis(200),
            rotor_timer: Duration::from_secs(0),

            copter_images: CopterImages::new(state),
            copter_state,

            shot_timeout: Duration::from_millis(110),
            shot_timer: Duration::from_secs(0),
            shots: vec![],

            enemies: vec![],
            spawn_enemies: false,
            enemy_texture: texture_from_text(state, "HURENSOHN", state.font_size, None),
            enemy_speed: state.window_width / 20.0,
            enemy_duration: Duration::from_millis(100),
            enemy_timer: Duration::from_secs(0),
        }
    }

    pub fn update(&mut self, state: &State) {
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

        self.handle_shots(state);

        self.spawn_enemies(state);
        self.update_enemies();

        let mut next_state: Option<CopterState> = None;
        // Update state dependant variables.
        match self.copter_state {
            CopterState::Flying {
                ref mut position,
                ref dest,
            } => {
                // Check, whether we reached our position
                if dest.sub(position.clone()).length() < 20.0 {
                    let copter_direction = if position.x > dest.x {
                        Side::Left
                    } else {
                        Side::Right
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

        self.draw_enemies();

        match self.copter_state {
            CopterState::Flying {
                ref mut position,
                ref dest,
            } => {
                let copter_direction = side(position, dest);

                let angle = match copter_direction {
                    Side::Left => -PI as f32 / 8.0,
                    Side::Right => PI as f32 / 8.0,
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
                ref timer,
                ref position,
                ref copter_direction,
                ..
            } => {
                // We animate hovering by following in a sinus curve depending on the time
                let mut current_rotation = (timer.as_millis() as f64 / 1000.0f64) as f32;
                // Speed up things a little
                current_rotation = current_rotation * 1.2;

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

    /// Update the enemy position and check collisions with shots.
    fn update_enemies(&mut self) {
        let copter_position = self.get_copter_position();
        //let enemies_to_remove = Vec::new();

        for enemy in self.enemies.iter_mut() {
            // Check if the enemy hit the player
            //if middle direction.length() <= 40

            let direction = copter_position - enemy.position;
            let distance = (direction / direction.length()) * self.enemy_speed * get_frame_time();
            enemy.position = enemy.position + distance;
        }
    }

    /// Update the enemy spawn timer and spawn enemies, if it's time.
    fn spawn_enemies(&mut self, state: &State) {
        if !self.spawn_enemies {
            return;
        }
        self.enemy_timer += delta_duration();

        // It isn't time yet.
        if self.enemy_timer < self.enemy_duration {
            return;
        }

        self.enemy_timer = Duration::from_secs(0);

        self.enemies
            .push(Enemy::new(random_position_outside_screen(state)));
    }

    /// Update the enemy spawn timer and spawn enemies, if it's time.
    fn draw_enemies(&self) {
        let copter_position = self.get_copter_position();
        for enemy in self.enemies.iter() {
            let direction = copter_position - enemy.position;
            draw_texture_ex(
                self.enemy_texture,
                enemy.position.x,
                enemy.position.y,
                RED,
                DrawTextureParams {
                    rotation: vec2_to_radian(direction),
                    flip_y: true,
                    ..Default::default()
                },
            );
        }
    }

    fn get_copter_position(&self) -> Vec2 {
        match self.copter_state {
            CopterState::Flying { position, .. } => position,
            CopterState::Hovering { position, .. } => position,
        }
    }

    /// Tick all shots and spawn new ones, if the mouse is down.
    pub fn handle_shots(&mut self, state: &State) {
        // Animate the shots
        let mut shots_to_remove = Vec::new();
        let mut enemies_to_remove = Vec::new();
        for (shot_index, shot) in self.shots.iter_mut().enumerate() {
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
                self.fire_shot(Vec2::new(state.mouse_position.0, state.mouse_position.1))
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
                    self.fire_shot(position);
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
    pub fn fire_shot(&mut self, dest: Vec2) {
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
