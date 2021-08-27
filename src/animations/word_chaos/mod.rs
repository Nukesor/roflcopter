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
                position: Vec2::new(
                    gen_range(100.0, state.window_width - 100.0),
                    gen_range(100.0, state.window_height - 100.0),
                ),
                acceleration: Vec2::new(gen_range(-200.0, 200.0), gen_range(-200.0, 200.0)),
                color: random_color(),
                angle: 0.0,
                angle_rotation: gen_range(0.1, 0.5),
            }],
            word_texture: texture_from_text(state, &word, false),
            current: word,
        }
    }

    /// Restart the animation, with a new word.
    pub fn next_word(&mut self, state: &State, word: &str) {
        self.words = vec![Word {
            position: Vec2::new(
                gen_range(100.0, state.window_width - 100.0),
                gen_range(100.0, state.window_height - 100.0),
            ),
            acceleration: Vec2::new(gen_range(-200.0, 200.0), gen_range(-200.0, 200.0)),
            length: word.len(),
            color: random_color(),
            angle: gen_range(0.0, 2.0 * PI),
            angle_rotation: gen_range(0.1, 0.2),
        }];
        self.word_texture = texture_from_text(state, &word, false);
        self.current = word.to_owned();
    }

    /// Update our word texture.
    /// This is necessary, if the screen get's resized.
    pub fn update_texture(&mut self, state: &State) {
        self.word_texture = texture_from_text(state, &self.current, false);
    }

    pub fn update(&mut self, state: &State) {
        let dt = get_frame_time();

        let mut new_words = vec![];
        let mut words_to_remove = vec![];
        let current_words = self.words.len();

        for (index, word) in self.words.iter_mut().enumerate() {
            word.angle = word.angle + word.angle_rotation * dt;
            word.position = word.position + word.acceleration * dt;
            let middle = word.mid_position(state);

            // Check collisions on all sides
            let mut collision = false;
            if middle.x > state.window_width + 1.0 {
                word.set_x_from_mid(state, state.window_width - 1.0);
                word.acceleration.x = -word.acceleration.x;
                collision = true;
            }
            if middle.x < -1.0 {
                word.set_x_from_mid(state, 1.0);
                word.acceleration.x = -word.acceleration.x;
                collision = true;
            }
            if middle.y > state.window_height + 1.0 {
                word.set_y_from_mid(state, state.window_height - 1.0);
                word.acceleration.y = -word.acceleration.y;
                collision = true;
            }
            if middle.y < -1.0 {
                word.set_y_from_mid(state, 1.0);
                word.acceleration.y = -word.acceleration.y;
                collision = true;
            }

            // Generic collision handler
            if collision {
                word.angle_rotation = gen_range(1.1, 1.4);
                // If the word is gone, schedule it for removal.
                // Otherwise, split it and spawn new ones.
                if word.length == 1 {
                    words_to_remove.push(index)
                } else {
                    word.length -= 1;

                    if (current_words + new_words.len()) < 50000 {
                        spawn_new_words(&mut new_words, &word);
                    }
                }
            }
        }
        // Append all new words.
        self.words.append(&mut new_words);

        if self.words.is_empty() {
            self.next_word(state, "Roflcopter");
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
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 20.0, WHITE);
        draw_text(
            &format!("Words: {}", self.words.len()),
            20.0,
            40.0,
            20.0,
            WHITE,
        );
    }
}

fn spawn_new_words(new_words: &mut Vec<Word>, word: &Word) {
    for _ in 0..1 {
        let mut new_word = Word { ..word.clone() };
        new_word.acceleration.x = new_word.acceleration.x * gen_range(1.1, 1.3);
        new_word.acceleration.y = new_word.acceleration.y * gen_range(1.1, 1.3);
        new_word.color = random_color();

        new_words.push(new_word);
    }
}
