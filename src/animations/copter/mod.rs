use std::time::Duration;

use macroquad::prelude::*;

mod copter;
mod draw;
mod enemy;
mod images;
mod shot;

use self::draw::*;
use self::enemy::*;
use self::images::*;
use self::shot::*;
use crate::helper::*;
use crate::state::State;

#[derive(Debug, Clone)]
pub struct CopterAnimation {
    pub textures: Textures,

    pub copter_state: CopterState,
    rotor_direction: Side,
    rotor_duration: Duration,
    rotor_timer: Duration,

    shot_timeout: Duration,
    shot_timer: Duration,
    shots: Vec<Shot>,

    enemies: Vec<Enemy>,
    spawn_enemies: bool,
    enemy_speed: f32,
    enemy_timer: Duration,
    enemy_duration: Duration,
}

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

            textures: Textures::new(state),
            copter_state,

            shot_timeout: Duration::from_millis(110),
            shot_timer: Duration::from_secs(0),
            shots: vec![],

            enemies: vec![],
            spawn_enemies: true,
            enemy_speed: state.window_width / 20.0,
            enemy_duration: Duration::from_millis(100),
            enemy_timer: Duration::from_secs(0),
        }
    }

    pub fn update(&mut self, state: &State) {
        self.update_shots(state);
        self.spawn_enemies(state);
        self.update_enemies();

        self.update_copter(state);
    }

    /// Draw the copter depending on the current animation state.
    pub fn draw(&self, state: &State) {
        for shot in self.shots.iter() {
            draw_shot(&self.textures, shot);
        }

        self.draw_enemies();
        self.draw_copter(state);
    }

    fn get_copter_position(&self) -> Vec2 {
        match self.copter_state {
            CopterState::Flying { position, .. } => position,
            CopterState::Hovering { position, .. } => position,
        }
    }
}
