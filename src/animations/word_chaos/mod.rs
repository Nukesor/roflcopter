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

    pub fn update(&mut self, _state: &State) {
        let dt = get_frame_time();
        for word in self.words.iter_mut() {
            word.angle = word.angle + word.angle_rotation * dt;
        }
    }

    pub fn draw(&self, state: &State) {
        for word in self.words.iter() {
            draw_circle(
                word.position.x,
                word.position.x,
                1.0,
                Color::new(1.0, 1.0, 1.0, 1.0),
            );
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
