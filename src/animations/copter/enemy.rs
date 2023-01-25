use std::time::Duration;

use macroquad::prelude::*;

use super::RoflcopterAnimation;
use crate::helper::*;
use crate::state::State;

#[derive(Debug, Clone)]
pub struct Enemy {
    pub position: Vec2,
    pub health: usize,
}

impl RoflcopterAnimation {
    /// Update the enemy position and check collisions with shots.
    pub fn update_enemies(&mut self) {
        let copter_position = self.get_copter_position();
        //let enemies_to_remove = Vec::new();

        for enemy in self.enemies.iter_mut() {
            // Check if the enemy hit the player
            //if middle direction.length() <= 40

            let direction = copter_position - enemy.position;
            let distance = (direction / direction.length()) * self.enemy_speed * get_frame_time();
            enemy.position += distance;
        }
    }

    /// Update the enemy spawn timer and spawn enemies, if it's time.
    pub fn spawn_enemies(&mut self, state: &State) {
        // Global flag to stop spawning enemies.
        if !self.spawn_enemies {
            return;
        }

        // Update timer and check if it's time to spawn a wave.
        self.enemy_wave_timer += delta_duration();
        if self.enemy_wave_timer < self.enemy_wave_timeout {
            return;
        } else {
            self.enemy_wave_timer = Duration::from_secs(0);
        }

        // Spawn enemies in a cluster around a random point outside the screen.
        let offset = state.window_width / 10.0;
        let cluster_center = random_position_outside_screen(state, offset);
        for _ in 0..self.enemy_wave_size {
            let position = cluster_center + random_vector_with_lenght(offset);
            self.enemies.push(Enemy {
                position,
                health: self.enemy_max_health,
            });
        }
    }

    /// Update the enemy spawn timer and spawn enemies, if it's time.
    pub fn draw_enemies(&self) {
        let copter_position = self.get_copter_position();
        for enemy in self.enemies.iter() {
            let direction = copter_position - enemy.position;
            let health_percent = enemy.health as f32 / self.enemy_max_health as f32;
            draw_texture_ex(
                self.textures.enemy,
                enemy.position.x,
                enemy.position.y,
                Color::new(1.0, 1.0 * health_percent, 1.0 * health_percent, 1.0),
                DrawTextureParams {
                    rotation: vec2_to_radian(direction),
                    flip_y: true,
                    ..Default::default()
                },
            );
        }
    }
}
