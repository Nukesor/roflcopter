use std::f32::consts::PI;

use macroquad::{prelude::*, rand::gen_range};

use crate::{
    animations::helper::{random_color, texture_from_text},
    state::State,
};

#[derive(Debug, Clone)]
pub struct Word {
    pub length: usize,
    pub position: Vec2,
    pub acceleration: Vec2,
    pub color: Color,
    pub angle: f32,
    pub angle_rotation: f32,
}

impl Word {
    pub fn mid_position(&self, state: &State) -> Vec2 {
        Vec2::new(
            self.position.x + self.length as f32 * state.font_dimensions.width / 2.0,
            self.position.y + state.font_dimensions.height / 2.0,
        )
    }

    pub fn set_x_from_mid(&mut self, state: &State, x: f32) {
        self.position.x = x - self.length as f32 * state.font_dimensions.width / 2.0;
    }

    pub fn set_y_from_mid(&mut self, state: &State, y: f32) {
        self.position.y = y - state.font_dimensions.height / 2.0;
    }
}

#[derive(Debug, Clone)]
pub struct WordChaosAnimation {
    pub words: Vec<Word>,
    pub current: String,
    pub word_texture: Texture2D,
}

impl WordChaosAnimation {
    pub fn new(state: &State) -> WordChaosAnimation {
        let word = "Roflcopter".to_owned();
        WordChaosAnimation {
            words: vec![Word {
                length: word.len(),
                position: Vec2::new(200.0, 200.0),
                acceleration: Vec2::new(100.0, 100.0),
                color: random_color(),
                angle: 0.0,
                angle_rotation: 0.5,
            }],
            word_texture: texture_from_text(state, &word, false),
            current: word,
        }
    }

    /// Restart the animation, with a new word.
    pub fn next_word(&mut self, state: &State, word: String) {
        self.words = vec![Word {
            position: Vec2::new(200.0, 200.0),
            acceleration: Vec2::new(100.0, 100.0),
            length: word.len(),
            color: random_color(),
            angle: gen_range(0.0, 2.0 * PI),
            angle_rotation: gen_range(0.1, 0.2),
        }];
        self.word_texture = texture_from_text(state, &word, false);
        self.current = word;
    }

    /// Update our word texture.
    /// This is necessary, if the screen get's resized.
    pub fn update_texture(&mut self, state: &State) {
        self.word_texture = texture_from_text(state, &self.current, false);
    }

    pub fn update(&mut self, state: &State) {
        let dt = get_frame_time();
        for word in self.words.iter_mut() {
            word.angle = word.angle + word.angle_rotation * dt;

            word.position = word.position + word.acceleration * dt;
            let middle = word.mid_position(state);

            if middle.x > state.window_width {
                word.set_x_from_mid(state, state.window_width);
                word.acceleration.x = -word.acceleration.x;
            }
            if middle.x < 0.0 {
                word.set_x_from_mid(state, 0.0);
                word.acceleration.x = -word.acceleration.x;
            }
            if middle.y > state.window_height {
                word.set_y_from_mid(state, state.window_height);
                word.acceleration.y = -word.acceleration.y;
            }
            if middle.y < 0.0 {
                word.set_y_from_mid(state, 0.0);
                word.acceleration.y = -word.acceleration.y;
            }
        }
    }

    pub fn draw(&self, state: &State) {
        for word in self.words.iter() {
            draw_texture_ex(
                self.word_texture,
                word.position.x,
                word.position.y,
                word.color,
                DrawTextureParams {
                    source: Some(Rect {
                        x: 0.0,
                        y: 0.0,
                        w: word.length as f32 * state.font_dimensions.width,
                        h: state.font_dimensions.height + state.font_dimensions.offset_y,
                    }),
                    rotation: word.angle + PI,
                    flip_y: true,
                    ..Default::default()
                },
            );
        }
    }
}
