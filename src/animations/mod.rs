pub mod copter;
pub mod helper;
pub mod wall;

use std::time::Duration;

use cgmath::Vector2;
pub use copter::*;
pub use helper::*;
pub use wall::animate_wall;

use crate::state::State;

#[derive(Debug, Clone)]
pub enum Animation {
    Wall(WallAnimation),
    Copter(CopterAnimation),
}

#[derive(Debug, Clone)]
pub struct WallAnimation {
    pub y_offset: f32,
    pub x_offset: f32,
}

#[derive(Debug, Clone)]
pub struct CopterAnimation {
    pub rotor_direction: Direction,
    pub rotor_duration: Duration,
    pub rotor_timer: Duration,
    pub copter_images: CopterImages,
    pub state: CopterState,
}

impl CopterAnimation {
    pub fn new(state: &State, position: Vector2<f32>) -> Animation {
        Animation::Copter(CopterAnimation {
            rotor_direction: Direction::Left,
            rotor_duration: Duration::from_millis(200),
            rotor_timer: Duration::from_secs(0),
            copter_images: CopterImages::new(state),
            state: CopterState::Hovering {
                duration: Duration::from_millis(1000),
                timer: Duration::from_secs(0),
                position,
                copter_direction: Direction::Left,
            },
            //state: CopterState::Flying {
            //    position,
            //    dest: Vector2::new(100.0, 100.0),
            //},
        })
    }
}
