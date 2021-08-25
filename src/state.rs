use std::time::Duration;

use macroquad::prelude::*;

use crate::color;

pub struct State {
    pub font: Font,
    pub font_size: u16,
    pub font_dimensions: TextDimensions,

    // The total time of the duration and current animation length.
    pub animation_length: Duration,
    pub animation_timer: Duration,

    pub word: String,
    /// For each character of the word, a color will be assigned.
    pub colors: Vec<Color>,
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

        let animation_length = Duration::from_secs(10);
        let animation_timer = Duration::from_secs(0);

        State {
            font,
            font_size,
            font_dimensions,
            animation_length,
            animation_timer,
            word,
            colors,
        }
    }
}
