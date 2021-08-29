use macroquad::prelude::Texture2D;

use super::draw::draw_raw_copter;
use crate::{helper::*, state::State};

/// A struct used to store dynamically generated images of the roflcopter.
#[derive(Debug, Clone)]
pub struct CopterImages {
    pub right_copter_right_rotor: Texture2D,
    pub right_copter_left_rotor: Texture2D,
    pub left_copter_right_rotor: Texture2D,
    pub left_copter_left_rotor: Texture2D,
    pub shot: Texture2D,
}

impl CopterImages {
    pub fn new(state: &State) -> CopterImages {
        CopterImages {
            right_copter_right_rotor: draw_raw_copter(state, Side::Right, Side::Right),
            right_copter_left_rotor: draw_raw_copter(state, Side::Right, Side::Left),
            left_copter_right_rotor: draw_raw_copter(state, Side::Left, Side::Right),
            left_copter_left_rotor: draw_raw_copter(state, Side::Left, Side::Left),
            shot: texture_from_text(state, &state.word, state.font_size, Some(&state.colors)),
        }
    }

    /// Dynamically update the copter images.
    /// This is necessary, if we dynamically change our font size.
    ///  For instance, during window resizes.
    pub fn update(&mut self, state: &State) {
        self.right_copter_right_rotor = draw_raw_copter(state, Side::Right, Side::Right);
        self.right_copter_left_rotor = draw_raw_copter(state, Side::Right, Side::Left);
        self.left_copter_right_rotor = draw_raw_copter(state, Side::Left, Side::Right);
        self.left_copter_left_rotor = draw_raw_copter(state, Side::Left, Side::Left);
    }

    pub fn copter_dimensions(&self) -> (f32, f32) {
        (
            self.right_copter_left_rotor.width(),
            self.right_copter_left_rotor.height(),
        )
    }

    /// Simple helper, which gets the correct texture for a copter and rotor orientation.
    pub fn get_for_directions(&self, copter_direction: &Side, rotor_direction: &Side) -> Texture2D {
        match copter_direction {
            Side::Right => match rotor_direction {
                Side::Right => self.right_copter_right_rotor.clone(),
                Side::Left => self.right_copter_left_rotor.clone(),
            },
            Side::Left => match rotor_direction {
                Side::Right => self.left_copter_right_rotor.clone(),
                Side::Left => self.left_copter_left_rotor.clone(),
            },
        }
    }
}
