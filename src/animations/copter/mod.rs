use std::time::Duration;

use macroquad::prelude::*;

mod draw;
mod enemy;
mod images;
mod roflcopter;
mod shot;

use self::enemy::*;
use self::images::*;
use self::shot::*;
use crate::helper::*;
use crate::state::State;

#[derive(Debug, Clone)]
pub struct RoflcopterAnimation {
    pub textures: Textures,

    pub roflcopter_state: RoflcopterState,
    rotor_direction: Side,
    rotor_duration: Duration,
    rotor_timer: Duration,

    shot_timeout: Duration,
    shot_timer: Duration,
    shots: Vec<Shot>,

    enemies: Vec<Enemy>,
    spawn_enemies: bool,
    enemy_speed: f32,
    enemy_max_health: usize,
    enemy_wave_size: usize,
    enemy_wave_timeout: Duration,
    enemy_wave_timer: Duration,
}

#[derive(Debug, Clone)]
pub enum RoflcopterState {
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

impl RoflcopterAnimation {
    pub fn new(state: &State, position: Vec2) -> RoflcopterAnimation {
        let copter_state = RoflcopterState::Flying {
            position,
            dest: Vec2::new(100.0, 100.0),
        };

        RoflcopterAnimation {
            rotor_direction: Side::Left,
            rotor_duration: Duration::from_millis(200),
            rotor_timer: Duration::from_secs(0),

            textures: Textures::new(state),
            roflcopter_state: copter_state,

            shot_timeout: Duration::from_millis(300),
            shot_timer: Duration::from_secs(0),
            shots: vec![],

            enemies: vec![],
            spawn_enemies: true,
            enemy_speed: state.window_width / 20.0,
            enemy_max_health: 3,
            enemy_wave_size: 12,
            enemy_wave_timeout: Duration::from_secs(10),
            enemy_wave_timer: Duration::from_secs(0),
        }
    }

    pub fn update(&mut self, state: &State) {
        self.update_shots(state);
        self.spawn_enemies(state);
        self.update_enemies();

        self.update_roflcopter(state);
    }

    /// Draw the copter depending on the current animation state.
    pub fn draw(&self, state: &State) {
        self.draw_shots();
        self.draw_enemies();
        self.draw_roflcopter(state);
    }

    fn get_copter_position(&self) -> Vec2 {
        match self.roflcopter_state {
            RoflcopterState::Flying { position, .. } => position,
            RoflcopterState::Hovering { position, .. } => position,
        }
    }
}
