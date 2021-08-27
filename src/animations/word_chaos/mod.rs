use std::{collections::HashMap, f32::consts::PI, time::Duration};

use macroquad::{prelude::*, rand::gen_range};

use crate::{
    animations::helper::{random_color, textures_from_text},
    state::State,
};

use super::helper::{delta_duration, rotate_vec2};

pub enum Collision {
    Top,
    Left,
    Bottom,
    Right,
}

#[derive(Debug, Clone)]
pub struct Word {
    pub length: usize,
    pub position: Vec2,
    pub acceleration: Vec2,
    pub color: Color,
    pub angle: f32,
    pub angle_rotation: f32,
    pub font_size: u16,
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
    pub texture_map: HashMap<u16, Texture2D>,

    pub spawn_timeout: Duration,
    pub spawn_timer: Duration,
}

impl WordChaosAnimation {
    pub fn new(state: &State) -> WordChaosAnimation {
        let word = "Roflcopter".to_owned();
        let textures = textures_from_text(state, &word, false);
        WordChaosAnimation {
            words: vec![Word {
                length: word.len(),
                position: Vec2::new(
                    gen_range(100.0, state.window_width - 100.0),
                    gen_range(100.0, state.window_height - 100.0),
                ),
                acceleration: Vec2::new(gen_range(100.0, 200.0), gen_range(100.0, 200.0)),
                color: random_color(),
                angle: 0.0,
                angle_rotation: gen_range(0.1, 0.5),
                font_size: state.font_size,
            }],
            texture_map: textures,
            spawn_timeout: Duration::from_millis(300),
            spawn_timer: Duration::from_millis(0),
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
            acceleration: Vec2::new(gen_range(100.0, 200.0), gen_range(100.0, 200.0)),
            length: word.len(),
            color: random_color(),
            angle: gen_range(0.0, 2.0 * PI),
            angle_rotation: gen_range(0.1, 0.2),
            font_size: state.font_size,
        }];
        self.texture_map = textures_from_text(state, &word, false);
        self.current = word.to_owned();
    }

    /// Restart the animation, with a new word.
    pub fn new_word_at_position(&mut self, state: &State, position: Vec2) {
        self.words.push(Word {
            position,
            acceleration: Vec2::new(gen_range(100.0, 200.0), gen_range(100.0, 200.0)),
            length: self.current.len(),
            color: random_color(),
            angle: gen_range(0.0, 2.0 * PI),
            angle_rotation: gen_range(0.1, 0.2),
            font_size: state.font_size,
        });
    }

    /// Update our word texture.
    /// This is necessary, if the screen get's resized.
    pub fn update_texture(&mut self, state: &State) {
        self.texture_map = textures_from_text(state, &self.current, false);
    }

    pub fn update(&mut self, state: &State) {
        let dt = get_frame_time();

        self.handle_mouse_click(state);

        let mut new_words = vec![];
        let mut words_to_remove = vec![];
        let current_words = self.words.len();
        let font_sizes: Vec<u16> = self.texture_map.keys().cloned().collect();

        for (index, word) in self.words.iter_mut().enumerate() {
            word.angle = word.angle + word.angle_rotation * dt;
            word.position = word.position + word.acceleration * dt;
            let middle = word.mid_position(state);

            let collision = detect_collision(middle, state);

            if let Some(collision) = collision {
                // Update the word's acceleration
                match collision {
                    Collision::Right => {
                        word.set_x_from_mid(state, state.window_width - 1.0);
                        word.acceleration.x = -word.acceleration.x;
                    }
                    Collision::Left => {
                        word.set_x_from_mid(state, 1.0);
                        word.acceleration.x = -word.acceleration.x;
                    }
                    Collision::Bottom => {
                        word.set_y_from_mid(state, state.window_height - 1.0);
                        word.acceleration.y = -word.acceleration.y;
                    }
                    Collision::Top => {
                        word.set_y_from_mid(state, 1.0);
                        word.acceleration.y = -word.acceleration.y;
                    }
                }

                // Make the word spin faster.
                word.angle_rotation *= gen_range(1.1, 1.4);

                // Slightly change acceleration
                word.acceleration = rotate_vec2(word.acceleration, gen_range(0.1, 0.4));
                word.acceleration = word.acceleration * gen_range(1.1, 1.2);

                // If the word is gone, schedule it for removal.
                // Otherwise, split it and spawn new ones.
                if word.length == 1 {
                    words_to_remove.push(index)
                } else {
                    word.length -= 1;

                    if (current_words + new_words.len()) < 50000 {
                        let mut new_word = get_new_word(&word, collision);
                        new_word.font_size = font_sizes[gen_range(0, font_sizes.len() - 1)];
                        new_words.push(new_word);
                    }
                }
            }
        }
        words_to_remove.reverse();
        for index in words_to_remove.iter() {
            self.words.remove(*index);
        }

        // Append all new words.
        self.words.append(&mut new_words);

        if self.words.is_empty() {
            self.next_word(state, "Roflcopter");
        }
    }

    pub fn draw(&self, _: &State) {
        for word in self.words.iter() {
            let texture = self.texture_map.get(&word.font_size).unwrap();
            let width = (texture.width() / self.current.len() as f32) * word.length as f32;
            draw_texture_ex(
                *texture,
                word.position.x,
                word.position.y,
                word.color,
                DrawTextureParams {
                    source: Some(Rect {
                        x: 0.0,
                        y: 0.0,
                        w: width,
                        h: texture.height(),
                    }),
                    rotation: word.angle + PI,
                    flip_y: true,
                    ..Default::default()
                },
            );
        }
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Word: {}", self.current), 20.0, 40.0, 20.0, WHITE);
        draw_text(
            &format!("Words: {}", self.words.len()),
            20.0,
            60.0,
            20.0,
            WHITE,
        );
    }

    fn handle_mouse_click(&mut self, state: &State) {
        self.spawn_timer += delta_duration();

        if self.spawn_timer > self.spawn_timeout {
            if is_mouse_button_down(MouseButton::Left) {
                self.new_word_at_position(
                    state,
                    Vec2::new(state.mouse_position.0, state.mouse_position.1),
                )
            }
            self.spawn_timer = Duration::from_secs(0);
        }
    }
}

fn detect_collision(middle: Vec2, state: &State) -> Option<Collision> {
    // Check collisions on all sides
    if middle.x > state.window_width + 1.0 {
        return Some(Collision::Right);
    } else if middle.x < -1.0 {
        return Some(Collision::Left);
    } else if middle.y > state.window_height + 1.0 {
        return Some(Collision::Bottom);
    } else if middle.y < -1.0 {
        return Some(Collision::Top);
    }

    None
}

fn get_new_word(word: &Word, collision: Collision) -> Word {
    let mut new_word = Word { ..word.clone() };
    new_word.acceleration.x = new_word.acceleration.x * gen_range(1.1, 1.3);
    new_word.acceleration.y = new_word.acceleration.y * gen_range(1.1, 1.3);

    match collision {
        Collision::Top => {
            new_word.acceleration.x = -1.0 * new_word.acceleration.x;
        }
        Collision::Bottom => {
            new_word.acceleration.x = -1.0 * new_word.acceleration.x;
        }
        Collision::Left => {
            new_word.acceleration.y = -1.0 * new_word.acceleration.y;
        }
        Collision::Right => {
            new_word.acceleration.y = -1.0 * new_word.acceleration.y;
        }
    }
    new_word.color = random_color();

    new_word
}
