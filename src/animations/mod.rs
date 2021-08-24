pub mod wall;

pub use wall::animate_wall;

pub enum Animation {
    Wall(WallAnimation),
}

pub struct WallAnimation {
    pub y_offset: f32,
    pub x_offset: f32,
}
