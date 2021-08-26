mod draw;
mod images;
mod raw_draw;
mod state;

pub use self::images::CopterImages;
pub use self::state::*;
use crate::state::State;

pub fn animate_copter(state: &State, animation: &mut CopterAnimation) {
    animation.update();
    animation.draw(state);
}
