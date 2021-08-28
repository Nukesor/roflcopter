pub mod copter;
pub mod helper;
pub mod wall;
pub mod word_chaos;

pub use copter::*;
pub use helper::*;
use macroquad::prelude::Vec2;
use roflcopter_snake_lib::game_state::GameState;
pub use wall::WallAnimation;

use crate::state::State;

use self::word_chaos::WordChaosAnimation;

pub enum Animation {
    Wall(WallAnimation),
    Copter(CopterAnimation),
    WordChaos(WordChaosAnimation),
    Snake(Vec<GameState>),
}

impl Animation {
    pub fn new_copter(state: &State, position: Vec2) -> Animation {
        Animation::Copter(CopterAnimation::new(state, position))
    }

    pub fn new_wall() -> Animation {
        Animation::Wall(WallAnimation::new())
    }
    pub fn new_word_chaos(state: &State) -> Animation {
        Animation::WordChaos(WordChaosAnimation::new(state))
    }

    pub fn new_snake() -> Animation {
        let mut instances = Vec::new();
        for _ in 0..1000 {
            instances.push(GameState::new())
        }
        Animation::Snake(instances)
    }
}
