use std::time::Duration;

use macroquad::prelude::*;

use super::CopterAnimation;
use crate::helper::*;
use crate::state::State;

#[derive(Debug, Clone)]
pub struct Enemy {
    pub position: Vec2,
    pub health: usize,
}

impl Enemy {
    pub fn new(position: Vec2) -> Self {
        Enemy {
            position,
            health: 2,
        }
    }
}

impl CopterAnimation {
    /// Update the enemy position and check collisions with shots.
    pub fn update_enemies(&mut self) {
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
    pub fn spawn_enemies(&mut self, state: &State) {
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
    pub fn draw_enemies(&self) {
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
}
