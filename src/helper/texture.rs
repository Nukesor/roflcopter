use std::collections::HashMap;

use macroquad::prelude::*;

use crate::state::State;

pub fn middle_texture_position(position: Vec2, texture: &Texture2D) -> Vec2 {
    position + Vec2::new(texture.width() / 2.0, texture.height() / 2.0)
}

/// Create a texture map of different font sizes.
/// All textures for font sizes between start_font_size -/+ offset will be generated.
pub fn textures_from_text(
    state: &State,
    word: &str,
    start_font_size: u16,
    offset: u16,
    colors: Option<&Vec<Color>>,
) -> HashMap<u16, Texture2D> {
    let mut texture_map = HashMap::new();
    for size in start_font_size - offset..start_font_size + offset {
        let texture = texture_from_text(state, word, size, colors);
        texture_map.insert(size, texture);
    }

    texture_map
}

/// Create a texture for a given text.
/// If no color is given, we default to white.
pub fn texture_from_text(
    state: &State,
    text: &str,
    font_size: u16,
    colors: Option<&Vec<Color>>,
) -> Texture2D {
    clear_background(Color::from_rgba(0, 0, 0, 0));
    let font_dimensions = measure_text("j", Some(state.font), font_size, 1.0);

    // Remember the amount of lines and the max character width.
    // We need this to calculate the rectangle that should be extracted from the
    // full screen later on.
    let mut max_y = font_dimensions.height;
    let mut max_x = 0.0;

    let mut x: f32;
    for line in text.lines() {
        x = 0.0;
        for (index, character) in line.chars().enumerate() {
            // Take the provided colors or fallback to white.
            let color = if let Some(colors) = colors {
                colors[index]
            } else {
                Color::new(1.0, 1.0, 1.0, 1.0)
            };

            draw_text_ex(
                &character.to_string(),
                x,
                max_y,
                TextParams {
                    font: state.font,
                    font_size,
                    font_scale: 1.0,
                    color,
                    ..Default::default()
                },
            );
            x += font_dimensions.width;
        }

        // Move the draw position to the next line
        max_y += font_dimensions.height;

        // Remember the longest line.
        if x > max_x {
            max_x = x;
        }
    }

    // Make a screenshot and extract the roflcopter from it.
    let image = get_screen_data();
    let image = image.sub_image(Rect {
        x: 0.0,
        y: image.height as f32 - max_y,
        w: max_x,
        h: max_y,
    });

    clear_background(BLACK);
    Texture2D::from_image(&image)
}
