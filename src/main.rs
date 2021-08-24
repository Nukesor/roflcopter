use macroquad::prelude::*;

mod animations;
mod color;
mod state;

use crate::animations::*;
use crate::state::State;

#[macroquad::main("Text")]
async fn main() {
    let state = State::new().await;
    let mut animation = Animation::Wall(WallAnimation {
        y_offset: 0.0,
        x_offset: 0.0,
    });

    loop {
        clear_background(BLACK);

        match animation {
            Animation::Wall(ref mut inner) => {
                animate_wall(&state, inner);
            }
        }

        next_frame().await
    }
}
