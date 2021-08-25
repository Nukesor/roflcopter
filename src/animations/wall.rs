use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;

use super::WallAnimation;
use crate::state::State;

pub fn animate_wall(state: &State, animation: &mut WallAnimation) {
    let window_height = screen_height();
    let window_width = screen_width();

    // This is used to simulate a constant upflow/downfloat simulation
    let mut line = calculate_offsets(&state, animation, window_width);

    // We start at -100, as lines can shift upwards slowly
    let mut used_height = -100.0;
    while (window_height + 100.0) > used_height {
        draw_line(&state, animation, used_height, line, window_width);
        line += 1;
        used_height += state.font_dimensions.height;
    }
}

/// Calculate the actual offsets depending on the delta time.
///
/// This function is responsible for the actual animation, by determining and updating the offset
/// to the original start position.
fn calculate_offsets(state: &State, animation: &mut WallAnimation, window_width: f32) -> usize {
    let start = SystemTime::now();
    let time = start.duration_since(UNIX_EPOCH).unwrap().as_millis() as f64;

    let dt = get_frame_time();

    // Calculate the amount that has been moved since the last frame.
    // We oscilate through a sinus curve every few seconds.
    // The sinus is offseted by 1, which helps us to stay in positive range.
    let y_movement_speed = state.font_dimensions.height * 20.0;
    let mut y_rate = ((time / 1000.0f64).sin() + 1.0) as f32;
    // Enforce a minimum movement rate of 0.2
    y_rate += y_rate.signum() * 0.2;

    let moved_amount = dt * y_rate * y_movement_speed;
    animation.y_offset = animation.y_offset + moved_amount;

    // Set the movement speed relative to the text glyph width
    let x_movement_speed = state.font_dimensions.width * 1.0;
    let x_rate = ((time * 0.3f64 / 1000.0f64).sin()) as f32;
    let moved_amount = dt * x_rate * x_movement_speed;
    animation.x_offset = animation.x_offset + moved_amount;

    // Prevent floating too far away, due to floating point imprecision
    if animation.x_offset < -window_width / 2.0 {
        animation.x_offset = -window_width / 2.0;
    } else if animation.x_offset > window_width / 2.0 {
        animation.x_offset = window_width / 2.0;
    }

    let line = animation.y_offset / state.font_dimensions.height;
    line as usize
}

/// Draw one single line.
/// Each line is drawn depending on the current animation offset, state and text dimensions.
fn draw_line(
    state: &State,
    animation: &WallAnimation,
    current_height: f32,
    line_count: usize,
    window_width: f32,
) {
    let glyph_width = state.font_dimensions.width;
    let word_length = state.word.len();

    // Calculate the word and color offset for the current line.
    let offset = line_count % word_length;
    let (word, colors) = offsetted_word_and_colors(state, offset);

    // Runner width variable for this line.
    let mut current_width: f32 = -window_width / 2.0;
    let mut color_offset = 0;
    loop {
        for character in word.chars() {
            // Exit condition, stop the loop, if the next char doesn't fit onto the screen
            if current_width + glyph_width > window_width * 1.5 {
                return;
            }

            // We have a smooth movement, which is why we move in
            let height = current_height - animation.y_offset % state.font_dimensions.height;
            let width = current_width - animation.x_offset * state.word.len() as f32 * 2.0;

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
            current_width += glyph_width;
            color_offset = (color_offset + 1) % state.word.len();
        }
        // Add a space between this and the next word
        current_width += glyph_width;
        color_offset = (color_offset + 1) % state.word.len();
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
