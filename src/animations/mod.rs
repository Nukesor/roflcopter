pub mod copter;
pub mod helper;
pub mod wall;

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
