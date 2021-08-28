use macroquad::prelude::Vec2;
use roflcopter_snake_lib::game_state::GameState;

pub use copter::*;
pub use helper::*;
pub use wall::WallAnimation;

use crate::state::State;

use self::word_chaos::WordChaosAnimation;

pub mod copter;
pub mod helper;
pub mod wall;
pub mod word_chaos;

pub enum Animation {
    Wall(WallAnimation),
    Copter(CopterAnimation),
    WordChaos(WordChaosAnimation),
    Snake(GameState),
}

impl Animation {
    pub fn new_copter(state: &State) -> Animation {
        let position = Vec2::new(state.window_width / 2.0, state.window_height / 2.0);
        Animation::Copter(CopterAnimation::new(state, position))
    }

    pub fn new_wall() -> Animation {
        Animation::Wall(WallAnimation::new())
    }
    pub fn new_word_chaos(state: &State) -> Animation {
        Animation::WordChaos(WordChaosAnimation::new(state))
    }

    pub fn new_snake() -> Animation {
        Animation::Snake(GameState::new(500))
    }
}
