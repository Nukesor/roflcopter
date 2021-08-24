use macroquad::prelude::*;

use crate::color;

pub struct State {
    pub font: Font,
    pub font_size: u16,
    pub font_dimensions: TextDimensions,

    pub word: String,
    pub colors: Vec<Color>,

    pub y_offset: f32,
    pub x_offset: f32,
}

impl State {
    pub async fn new() -> Self {
        let font = load_ttf_font("fonts/RobotoMono-SemiBold.ttf")
            .await
            .expect("Font couldn't be loaded");
        let font_size = 30;
        let font_dimensions = measure_text("j", Some(font), font_size, 1.0);

        let word = "ROFLCOPTER".to_string();
        let mut colors = color::create_colors();
        colors.truncate(word.len());

        State {
            font,
            font_size,
            font_dimensions,
            word,
            colors,
            y_offset: 0.0,
            x_offset: 0.0,
        }
    }
}
