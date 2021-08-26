pub mod copter;
pub mod helper;
pub mod wall;

pub use copter::*;
pub use helper::*;
pub use wall::WallAnimation;

#[derive(Debug, Clone)]
pub enum Animation {
    Wall(WallAnimation),
    Copter(CopterAnimation),
}
