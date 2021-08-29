use macroquad::prelude::{Texture2D, Vec2};

use super::draw::generate_copter_texture;
use crate::{helper::*, state::State};

/// A struct used to store dynamically generated images of the roflcopter.
#[derive(Debug, Clone)]
pub struct Textures {
    pub right_copter_right_rotor: Texture2D,
    pub right_copter_left_rotor: Texture2D,
    pub left_copter_right_rotor: Texture2D,
    pub left_copter_left_rotor: Texture2D,
    pub shot: Texture2D,
    pub enemy: Texture2D,
}

impl Textures {
    pub fn new(state: &State) -> Textures {
        let enemy = "===>";

        Textures {
            right_copter_right_rotor: generate_copter_texture(state, Side::Right, Side::Right),
            right_copter_left_rotor: generate_copter_texture(state, Side::Right, Side::Left),
            left_copter_right_rotor: generate_copter_texture(state, Side::Left, Side::Right),
            left_copter_left_rotor: generate_copter_texture(state, Side::Left, Side::Left),
            shot: texture_from_text(state, "=>", state.font_size, None),
            enemy: texture_from_text(state, enemy, state.font_size, None),
        }
    }

    /// Dynamically update the copter images.
    /// This is necessary, if we dynamically change our font size.
    ///  For instance, during window resizes.
    pub fn update(&mut self, state: &State) {
        self.right_copter_right_rotor = generate_copter_texture(state, Side::Right, Side::Right);
        self.right_copter_left_rotor = generate_copter_texture(state, Side::Right, Side::Left);
        self.left_copter_right_rotor = generate_copter_texture(state, Side::Left, Side::Right);
        self.left_copter_left_rotor = generate_copter_texture(state, Side::Left, Side::Left);
    }

    pub fn copter_dimensions(&self) -> Vec2 {
        Vec2::new(
            self.right_copter_left_rotor.width(),
            self.right_copter_left_rotor.height(),
        )
    }

    pub fn texture(&self) -> &Texture2D {
        &self.right_copter_right_rotor
    }

    /// Simple helper, which gets the correct texture for a copter and rotor orientation.
    pub fn get_for_directions(
        &self,
        copter_direction: &Side,
        rotor_direction: &Side,
    ) -> &Texture2D {
        match copter_direction {
            Side::Right => match rotor_direction {
                Side::Right => &self.right_copter_right_rotor,
                Side::Left => &self.right_copter_left_rotor,
            },
            Side::Left => match rotor_direction {
                Side::Right => &self.left_copter_right_rotor,
                Side::Left => &self.left_copter_left_rotor,
            },
        }
    }
}
