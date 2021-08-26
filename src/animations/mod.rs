pub mod copter;
pub mod helper;
pub mod wall;

pub use copter::*;
pub use helper::*;
use macroquad::prelude::Vec2;
pub use wall::WallAnimation;

use crate::state::State;

#[derive(Debug, Clone)]
pub enum Animation {
    Wall(WallAnimation),
    Copter(CopterAnimation),
}

impl Animation {
    pub fn new_copter(state: &State, position: Vec2) -> Animation {
        Animation::Copter(CopterAnimation::new(state, position))
    }

    pub fn new_wall() -> Animation {
        Animation::Wall(WallAnimation::new())
    }
}
