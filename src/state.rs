use std::{ops::Add, time::Duration};

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::{
    animations::{
        helper::{delta_duration, direction, Direction},
        Animation, CopterState,
    },
    color,
    shaders::load_shaders,
};

pub struct Transition {
    pub timer: Duration,
    pub phase: Phase,
}

#[derive(Debug)]
pub enum Phase {
    In,
    Out,
}

/// The percentile of the screen height, a single char should occupy.
static RELATIVE_FONT_SIZE: f32 = 60.0;

pub struct State {
    pub word: String,
    pub font: Font,
    pub font_size: u16,
    pub font_dimensions: TextDimensions,
    /// For each character of the word, a color will be assigned.
    pub colors: Vec<Color>,

    /// The total time of the duration and current animation length.
    pub animation_duration: Duration,
    pub animation_timer: Duration,

    /// How long a transition should take
    pub transition_duration: Duration,
    /// The current state of the transition, if one is active
    pub transition: Option<Transition>,
    /// A black screen, which is used to simulate transitions between animations.
    pub black_screen: Texture2D,

    pub window_height: f32,
    pub window_width: f32,
    pub mouse_position: (f32, f32),
    pub shaders: Vec<Material>,
}

impl State {
    pub async fn new() -> Self {
        let window_height = screen_height();
        let window_width = screen_width();

        let font = load_ttf_font("fonts/RobotoMono-SemiBold.ttf")
            .await
            .expect("Font couldn't be loaded");
        let font_size = (window_height / RELATIVE_FONT_SIZE) as u16;
        let font_dimensions = measure_text("j", Some(font), font_size, 1.0);

        let word = "ROFLCOPTER".to_string();
        let mut colors = color::create_colors();
        colors.truncate(word.len());

        let animation_duration = Duration::from_secs(gen_range(10, 25));
        let animation_timer = Duration::from_secs(0);

        // Start the phase in transition animation
        let transition = Some(Transition {
            timer: Duration::from_secs(0),
            phase: Phase::In,
        });

        State {
            word,
            font,
            font_size,
            font_dimensions,
            colors,

            animation_duration,

            animation_timer,
            transition_duration: Duration::from_secs(2),
            transition,
            black_screen: Texture2D::empty(),

            window_height,
            window_width,
            mouse_position: mouse_position(),
            shaders: load_shaders(),
        }
    }

    pub fn update(&mut self, animation: &mut Animation) -> Option<Animation> {
        // Check if the window has been resized and update stuff accordingly.
        self.handle_window_resize(animation);
        self.handle_mouse_update(animation);

        let mut next_animation: Option<Animation> = None;
        let delta_time = delta_duration();

        // Tick the timer for the current animation.
        self.animation_timer = self.animation_timer.add(delta_time);

        // There's currently a transition running. Tick it
        if let Some(ref mut transition) = self.transition {
            if transition.timer > self.transition_duration {
                self.transition = None;
            } else {
                transition.timer = transition.timer.add(delta_time);
            }
        } else if self.animation_timer > self.animation_duration.add(self.transition_duration) {
            // There's no transition, check if we should start one.
            // This should be done, if the animation is finished.
            self.transition = Some(Transition {
                timer: Duration::from_secs(0),
                phase: Phase::Out,
            })
        }

        // The current transition has finished and the transition animation is done.
        if self.animation_timer > self.animation_duration.add(self.transition_duration * 2) {
            next_animation = Some(match animation {
                Animation::Wall(_) => Animation::new_copter(
                    &self,
                    Vec2::new(self.window_width / 2.0, self.window_height / 2.0),
                ),
                Animation::Copter(_) => Animation::new_wall(),
            });
            self.animation_timer = Duration::from_secs(0);
            self.animation_duration = Duration::from_secs(gen_range(8, 15));

            // Start the phase in transition animation
            self.transition = Some(Transition {
                timer: Duration::from_secs(0),
                phase: Phase::In,
            })
        }

        next_animation
    }

    pub fn handle_mouse_update(&mut self, animation: &mut Animation) {
        let (x, y) = mouse_position();

        if x == self.mouse_position.0 || y == self.mouse_position.1 {
            return;
        }

        self.mouse_position = (x, y);

        match animation {
            // While in copter mode, the copter should follow the mouse.
            Animation::Copter(ref mut copter) => match copter.copter_state {
                CopterState::Hovering { position, .. } => {
                    copter.copter_state = CopterState::Flying {
                        position: position.clone(),
                        dest: Vec2::new(x, y),
                    };
                }
                CopterState::Flying {
                    ref mut dest,
                    ref position,
                    ..
                } => {
                    let dimensions = copter.copter_images.copter_dimensions();
                    let new_dest = match direction(position, &dest) {
                        Direction::Left => Vec2::new(x, y - dimensions.1 / 2.0),
                        Direction::Right => Vec2::new(x - dimensions.0, y - dimensions.1 / 2.0),
                    };
                    *dest = new_dest;
                }
            },
            _ => {}
        }
    }

    /// Check whether the window size changed.
    /// If that's the case, update everything, that is depending on that size.
    pub fn handle_window_resize(&mut self, animation: &mut Animation) {
        let height = screen_height();
        let width = screen_width();

        // The screen changed. Update stuff, that depends on the current screen resolution
        if height != self.window_height || width != self.window_width {
            self.window_height = height;
            self.window_width = width;

            self.font_size = (self.window_height / RELATIVE_FONT_SIZE) as u16;
            self.font_dimensions = measure_text("j", Some(self.font), self.font_size, 1.0);

            // Grab an updated transition screen
            self.grab_black_screen();

            match animation {
                Animation::Wall(_) => {}
                Animation::Copter(inner) => inner.copter_images.update(self),
            }
        }
    }

    pub fn grab_black_screen(&mut self) {
        clear_background(BLACK);
        let image = get_screen_data();
        self.black_screen = Texture2D::from_image(&image);
    }
}
