use std::time::Duration;

use macroquad::{prelude::*, rand::gen_range};

use crate::state::State;

#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
}

pub fn delta_duration() -> Duration {
    let dt = (get_frame_time() * 1000.0 * 1000.0) as u64;
    Duration::from_micros(dt)
}

pub fn direction(src: &Vec2, dest: &Vec2) -> Direction {
    if src.x > dest.x {
        Direction::Left
    } else {
        Direction::Right
    }
}

pub fn random_color() -> Color {
    Color::from_rgba(gen_range(0, 255), gen_range(0, 255), gen_range(0, 255), 255)
}

pub fn texture_from_text(state: &State, word: &String, rainbow: bool) -> Texture2D {
    clear_background(BLACK);

    let mut x = 0.0;
    for (index, character) in word.chars().enumerate() {
        let color = if rainbow {
            state.colors[index]
        } else {
            Color::new(1.0, 1.0, 1.0, 1.0)
        };

        draw_text_ex(
            &character.to_string(),
            x,
            state.font_dimensions.height,
            TextParams {
                font: state.font,
                font_size: state.font_size,
                font_scale: 1.0,
                color,
                ..Default::default()
            },
        );
        x += state.font_dimensions.width;
    }

    // Make a screenshot and extract the roflcopter from it.
    let image = get_screen_data();
    let image = image.sub_image(Rect {
        x: 0.0,
        y: image.height as f32
            - state.font_dimensions.height
            - state.font_dimensions.offset_y / 2.0,
        w: x,
        h: state.font_dimensions.height + state.font_dimensions.offset_y / 2.0,
    });

    clear_background(BLACK);
    Texture2D::from_image(&image)
}
