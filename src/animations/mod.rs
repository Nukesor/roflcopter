pub mod copter;
pub mod helper;
pub mod wall;

use std::time::Duration;

pub use copter::*;
pub use helper::*;
pub use wall::animate_wall;

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
    pub state: CopterState,
}

impl CopterAnimation {
    pub fn new(position: Position) -> Animation {
        Animation::Copter(CopterAnimation {
            rotor_direction: Direction::Left,
            rotor_duration: Duration::from_millis(200),
            rotor_timer: Duration::from_secs(0),
            //state: CopterState::Hovering {
            //    duration: Duration::from_millis(1000),
            //    timer: Duration::from_secs(0),
            //    position,
            //},
            state: CopterState::Flying {
                position,
                dest: Position { x: 100.0, y: 100.0 },
            },
        })
    }
}
