use macroquad::prelude::Texture2D;

use super::raw_draw::draw_raw_copter;
use crate::{animations::helper::Direction, state::State};

/// A struct used to store dynamically generated images of the roflcopter.
#[derive(Debug, Clone)]
pub struct CopterImages {
    pub right_copter_right_rotor: Texture2D,
    pub right_copter_left_rotor: Texture2D,
    pub left_copter_right_rotor: Texture2D,
    pub left_copter_left_rotor: Texture2D,
}

impl CopterImages {
    pub fn new(state: &State) -> CopterImages {
        CopterImages {
            right_copter_right_rotor: draw_raw_copter(state, Direction::Right, Direction::Right),
            right_copter_left_rotor: draw_raw_copter(state, Direction::Right, Direction::Left),
            left_copter_right_rotor: draw_raw_copter(state, Direction::Left, Direction::Right),
            left_copter_left_rotor: draw_raw_copter(state, Direction::Left, Direction::Left),
        }
    }

    /// Dynamically update the copter images.
    /// This is necessary, if we dynamically change our font size.
    ///  For instance, during window resizes.
    pub fn update(&mut self, state: &State) {
        self.right_copter_right_rotor = draw_raw_copter(state, Direction::Right, Direction::Right);
        self.right_copter_left_rotor = draw_raw_copter(state, Direction::Right, Direction::Left);
        self.left_copter_right_rotor = draw_raw_copter(state, Direction::Left, Direction::Right);
        self.left_copter_left_rotor = draw_raw_copter(state, Direction::Left, Direction::Left);
    }

    /// Simple helper, which gets the correct texture for a copter and rotor orientation.
    pub fn get_for_directions(
        &self,
        copter_direction: &Direction,
        rotor_direction: &Direction,
    ) -> Texture2D {
        match copter_direction {
            Direction::Right => match rotor_direction {
                Direction::Right => self.right_copter_right_rotor.clone(),
                Direction::Left => self.right_copter_left_rotor.clone(),
            },
            Direction::Left => match rotor_direction {
                Direction::Right => self.left_copter_right_rotor.clone(),
                Direction::Left => self.left_copter_left_rotor.clone(),
            },
        }
    }
}
