use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;

use crate::state::State;

mod shader;

#[derive(Debug, Clone)]
pub struct WallAnimation {
    y_offset: f32,
    x_offset: f32,
}

impl WallAnimation {
    pub fn new() -> WallAnimation {
        WallAnimation {
            y_offset: 0.0,
            x_offset: 0.0,
        }
    }

    /// Calculate the actual offsets depending on the delta time.
    ///
    /// This function is responsible for the actual animation, by determining and updating the offset
    /// to the original start position.
    pub fn update(&mut self, state: &State) {
        // The current system time, determines the current x/y movement rate.
        let start = SystemTime::now();
        let time = start.duration_since(UNIX_EPOCH).unwrap().as_millis() as f64;

        // The delta time in combination with the movement rate is used to determine the moved
        // amount.
        let dt = get_frame_time();

        // Calculate the amount that has been moved since the last frame.
        // We oscilate through a sinus curve every few seconds.
        // The sinus is offseted by 1, which helps us to stay in positive range.
        let y_movement_speed = state.font_dimensions.height * 20.0;
        let mut y_rate = ((time / 1000.0f64).sin() + 1.0) as f32;
        // Enforce a minimum movement rate of 0.2
        y_rate += y_rate.signum() * 0.2;

        let moved_amount = dt * y_rate * y_movement_speed;
        self.y_offset = self.y_offset + moved_amount;

        // Set the movement speed relative to the text glyph width.
        let x_movement_speed = state.font_dimensions.width * 1.0;
        let x_rate = ((time * 0.3f64 / 1000.0f64).sin()) as f32;
        let moved_amount = dt * x_rate * x_movement_speed;
        self.x_offset = self.x_offset + moved_amount;

        // Prevent floating too far away, due to floating point imprecision
        if self.x_offset < -state.window_width / 2.0 {
            self.x_offset = -state.window_width / 2.0;
        } else if self.x_offset > state.window_width / 2.0 {
            self.x_offset = state.window_width / 2.0;
        }
    }

    /// Draw the roflcopter wall everything to the canvas.
    pub fn draw(&self, state: &State) {
        let mut line = (self.y_offset / state.font_dimensions.height) as usize;

        // We start at -100, as lines can shift upwards slowly
        let mut used_height = -100.0;
        while (state.window_height + 100.0) > used_height {
            self.draw_line(&state, used_height, line);
            line += 1;
            used_height += state.font_dimensions.height;
        }

        let ultra_rofl_size = (state.window_width / 5.0) as u16;
        let word_size = measure_text("ROFL", Some(state.font), ultra_rofl_size, 1.0);
        let mut ultra_rofl_position =
            Vec2::new(state.window_width / 2.0, state.window_height / 2.0);
        ultra_rofl_position.x -= word_size.width / 2.0;

        draw_text_ex(
            "ROFL",
            ultra_rofl_position.x,
            ultra_rofl_position.y,
            TextParams {
                font: state.font,
                font_size: ultra_rofl_size,
                font_scale: 1.0,
                color: BLACK,
                ..Default::default()
            },
        );

        shader::draw_shader(state);
    }

    /// Draw one single line.
    /// Each line is drawn depending on the current animation offset, state and text dimensions.
    fn draw_line(&self, state: &State, current_height: f32, line_count: usize) {
        let glyph_width = state.font_dimensions.width;
        let word_length = state.word.len();

        // Calculate the word and color offset for the current line.
        let offset = line_count % word_length;
        let (word, colors) = offsetted_word_and_colors(state, offset);

        // Runner width variable for this line.
        let mut current_width: f32 = -state.window_width;
        let mut color_offset = 0;
        loop {
            for character in word.chars() {
                // Exit condition, stop the loop, if the next char doesn't fit onto the screen
                if current_width + glyph_width > state.window_width * 2.0 {
                    return;
                }

                // We have a smooth movement, which is why we move in
                let height = current_height - self.y_offset % state.font_dimensions.height;
                let width = current_width - self.x_offset * state.word.len() as f32 * 2.0;

                // Don't draw the char, if it cannot be seen anyway.
                let mut skip = false;
                if width < 0.0 - state.font_dimensions.width {
                    skip = true;
                } else if width > state.window_width {
                    skip = true;
                } else if height < 0.0 - state.font_dimensions.height {
                    skip = true;
                } else if height > state.window_height + state.font_dimensions.height {
                    skip = true;
                }

                if !skip {
                    // Draw the character at the next position.
                    draw_text_ex(
                        &character.to_string(),
                        width,
                        height,
                        TextParams {
                            font: state.font,
                            font_size: state.font_size,
                            font_scale: 1.0,
                            color: colors[color_offset],
                            ..Default::default()
                        },
                    );
                }
                current_width += glyph_width;
                color_offset = (color_offset + 1) % state.word.len();
            }
            // Add a space between this and the next word
            current_width += glyph_width;
            color_offset = (color_offset + 1) % state.word.len();
        }
    }
}

/// Each line is offsetted to the previous by one.
/// This calculates the color and character offset for the current line and prepares
/// it for further usage.
fn offsetted_word_and_colors(state: &State, offset: usize) -> (String, Vec<Color>) {
    let mut word_start = state.word.to_owned();
    let mut word_end = word_start.split_off(offset);
    word_end.push_str(&word_start);

    let mut colors_start = state.colors.clone();
    let mut colors_end = colors_start.split_off(offset);
    colors_end.append(&mut colors_start);

    (word_end, colors_end)
}
