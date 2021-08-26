mod animation;
mod draw;
mod images;

pub use self::animation::*;
pub use self::images::CopterImages;
use crate::state::State;

pub fn animate_copter(state: &State, animation: &mut CopterAnimation) {
    animation.update();
    animation.draw(state);
}
