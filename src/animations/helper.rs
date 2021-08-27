use std::{collections::HashMap, time::Duration};

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

pub fn vec2_to_radian(vec: Vec2) -> f32 {
    vec.y.atan2(vec.x)
}

pub fn rotate_vec2(vec: Vec2, angle: f32) -> Vec2 {
    let x = vec.x * angle.cos() - vec.y * angle.sin();
    let y = vec.x * angle.sin() + vec.y * angle.cos();

    Vec2::new(x, y)
}

pub fn textures_from_text(state: &State, word: &str, rainbow: bool) -> HashMap<u16, Texture2D> {
    let mut texture_map = HashMap::new();
    for size in state.font_size - 5..state.font_size + 5 {
        let texture = texture_from_text(state, word, rainbow, size);
        texture_map.insert(size, texture);
    }

    texture_map
}

pub fn texture_from_text(state: &State, word: &str, rainbow: bool, font_size: u16) -> Texture2D {
    clear_background(Color::from_rgba(0, 0, 0, 0));

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
                font_size,
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
