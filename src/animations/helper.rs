use std::{collections::HashMap, f32::consts::PI, time::Duration};

use macroquad::{
    prelude::*,
    rand::{gen_range, ChooseRandom},
};

use crate::state::State;

pub enum Direction {
    Top,
    Left,
    Bottom,
    Right,
}

#[derive(Debug, Clone)]
pub enum Side {
    Left,
    Right,
}

pub fn delta_duration() -> Duration {
    let dt = (get_frame_time() * 1000.0 * 1000.0) as u64;
    Duration::from_micros(dt)
}

pub fn side(src: &Vec2, dest: &Vec2) -> Side {
    if src.x > dest.x {
        Side::Left
    } else {
        Side::Right
    }
}

pub fn random_color() -> Color {
    Color::from_rgba(gen_range(0, 255), gen_range(0, 255), gen_range(0, 255), 255)
}

pub fn vec2_to_radian(vec: Vec2) -> f32 {
    vec.y.atan2(vec.x)
}

pub fn random_vector_with_lenght(length: f32) -> Vec2 {
    let vec = Vec2::new(length, 0.0);
    rotate_vec2(vec, gen_range(0.0, PI * 2.0))
}

pub fn random_position_on_screen(state: &State) -> Vec2 {
    Vec2::new(
        gen_range(100.0, state.window_width - 100.0),
        gen_range(100.0, state.window_height - 100.0),
    )
}

pub fn random_position_outside_screen(state: &State) -> Vec2 {
    let mut position = Vec2::new(
        gen_range(0.0, state.window_width),
        gen_range(0.0, state.window_height),
    );

    let directions = vec![
        Direction::Top,
        Direction::Bottom,
        Direction::Left,
        Direction::Right,
    ];
    let direction = directions.choose().expect("Failed to get random direction");

    match direction {
        Direction::Top => position.y = -100.0,
        Direction::Bottom => position.y = state.window_height + 100.0,
        Direction::Left => position.x = -100.0,
        Direction::Right => position.x = state.window_width + 100.0,
    }

    position
}

pub fn rotate_vec2(vec: Vec2, angle: f32) -> Vec2 {
    let x = vec.x * angle.cos() - vec.y * angle.sin();
    let y = vec.x * angle.sin() + vec.y * angle.cos();

    Vec2::new(x, y)
}

pub fn textures_from_text(
    state: &State,
    word: &str,
    rainbow: bool,
    start_font_size: u16,
) -> HashMap<u16, Texture2D> {
    let mut texture_map = HashMap::new();
    for size in start_font_size - 5..start_font_size + 5 {
        let texture = texture_from_text(state, word, rainbow, size);
        texture_map.insert(size, texture);
    }

    texture_map
}

pub fn texture_from_text(state: &State, word: &str, rainbow: bool, font_size: u16) -> Texture2D {
    clear_background(Color::from_rgba(0, 0, 0, 0));
    let font_dimensions = measure_text("j", Some(state.font), font_size, 1.0);

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
