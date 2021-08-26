use std::{ops::Add, time::Duration};

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::{
    animations::{helper::delta_duration, Animation, CopterAnimation, WallAnimation},
    color,
};

pub struct State {
    pub font: Font,
    pub font_size: u16,
    pub font_dimensions: TextDimensions,

    // The total time of the duration and current animation length.
    pub animation_duration: Duration,
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

        let animation_duration = Duration::from_secs(10);
        let animation_timer = Duration::from_secs(0);

        State {
            font,
            font_size,
            font_dimensions,
            animation_duration,
            animation_timer,
            word,
            colors,
        }
    }

    pub fn update(&mut self, animation: &Animation) -> Option<Animation> {
        let window_height = screen_height();
        let window_width = screen_width();

        // Tick the timer for the current animation.
        self.animation_timer = self.animation_timer.add(delta_duration());

        // The current animation finished, start the next one.
        let mut next_animation: Option<Animation> = None;
        if self.animation_timer > self.animation_duration {
            next_animation = Some(match animation {
                Animation::Wall(_) => {
                    CopterAnimation::new(&self, Vec2::new(window_width / 2.0, window_height / 2.0))
                }
                Animation::Copter(_) => Animation::Wall(WallAnimation {
                    y_offset: 0.0,
                    x_offset: 0.0,
                }),
            });
            self.animation_timer = Duration::from_secs(0);
            self.animation_duration = Duration::from_secs(gen_range(8, 15));
        }

        next_animation
    }
}
