use std::collections::HashMap;

use macroquad::prelude::*;

use crate::state::State;

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
pub fn texture_from_text(
    state: &State,
    word: &str,
    font_size: u16,
    colors: Option<&Vec<Color>>,
) -> Texture2D {
    clear_background(Color::from_rgba(0, 0, 0, 0));
    let font_dimensions = measure_text("j", Some(state.font), font_size, 1.0);

    let mut x = 0.0;
    for (index, character) in word.chars().enumerate() {
        let color = if let Some(colors) = colors {
            colors[index]
        } else {
            Color::new(1.0, 1.0, 1.0, 1.0)
        };

        draw_text_ex(
            &character.to_string(),
            x,
            font_dimensions.height,
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

    // Make a screenshot and extract the roflcopter from it.
    let image = get_screen_data();
    let image = image.sub_image(Rect {
        x: 0.0,
        y: image.height as f32 - font_dimensions.height - font_dimensions.offset_y / 2.0,
        w: x,
        h: font_dimensions.height + font_dimensions.offset_y / 2.0,
    });

    clear_background(BLACK);
    Texture2D::from_image(&image)
}
