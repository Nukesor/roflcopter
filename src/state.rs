use std::{fs::read_to_string, ops::Add, path::Path, time::Duration};

use macroquad::{prelude::*, rand::ChooseRandom};

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

    /// A wordlist of random words that can be picked.
    pub wordlist: Vec<String>,

    /// Whether it's time to skip the current animation.
    pub show_debug: bool,
    /// Whether it's time to skip the current animation.
    skip_animation: bool,
    halt_animation_changes: bool,

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

        let mut wordlist = vec![word.clone()];
        let wordlist_path = Path::new("./wordlist.txt");
        if wordlist_path.exists() {
            let content = read_to_string(wordlist_path).expect("Failed while reading wordlist.");
            wordlist = content.split('\n').map(|word| word.to_owned()).collect();
        }

        //let animation_duration = Duration::from_secs(gen_range(10, 25));
        let animation_duration = Duration::from_secs(80);
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

            wordlist,

            show_debug: false,
            skip_animation: false,
            halt_animation_changes: false,

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
        self.handle_key_presses();

        let mut next_animation: Option<Animation> = None;
        let delta_time = delta_duration();

        if !self.halt_animation_changes {
            // Tick the timer for the current animation.
            self.animation_timer = self.animation_timer.add(delta_time);
        }

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
            self.skip_animation = true;
        }

        // Switch to the next animation.
        if self.skip_animation {
            next_animation = Some(match animation {
                Animation::Wall(_) => Animation::new_copter(self),
                Animation::Copter(_) => Animation::new_word_chaos(self),
                Animation::WordChaos(_) => Animation::new_snake(),
                Animation::Snake(_) => Animation::new_copter(self),
            });

            self.skip_animation = false;
            self.animation_timer = Duration::from_secs(0);

            // Start the phase in transition animation
            self.transition = Some(Transition {
                timer: Duration::from_secs(0),
                phase: Phase::In,
            });
        }

        next_animation
    }

    /// Animation independant draw logic.
    /// This includes animation transition logic.
    pub fn draw(&self) {
        if let Some(ref transition) = self.transition {
            // Calculate the gradiant, depending on the current state of the transition and,
            // whether it's a phase in or a phase out.
            let gradiant = (transition.timer.as_millis() as f64
                / self.transition_duration.as_millis() as f64) as f32;
            let gradiant = match transition.phase {
                Phase::In => 1.0 - gradiant,
                Phase::Out => gradiant,
            };

            draw_texture_ex(
                self.black_screen,
                0.0,
                0.0,
                Color::new(0.0, 0.0, 0.0, gradiant),
                DrawTextureParams {
                    flip_y: true,
                    ..Default::default()
                },
            )
        }

        if self.show_debug {
            draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 20.0, WHITE);
            draw_text(
                &format!("Duration: {:?}", self.animation_duration),
                20.0,
                40.0,
                20.0,
                WHITE,
            );
            draw_text(
                &format!("Timer: {:.2?}", self.animation_timer),
                20.0,
                60.0,
                20.0,
                WHITE,
            );
        }
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
                Animation::WordChaos(inner) => inner.update_texture(&self),
                Animation::Snake(_) => {}
            }
        }
    }

    fn handle_key_presses(&mut self) {
        if is_key_pressed(macroquad::prelude::KeyCode::D) {
            self.show_debug = !self.show_debug;
        }
        if is_key_pressed(macroquad::prelude::KeyCode::S) {
            self.skip_animation = true;
        }

        // Halt animations switches with H
        if is_key_pressed(macroquad::prelude::KeyCode::H) {
            self.halt_animation_changes = !self.halt_animation_changes;
        }

        // Change the animation duration by one sec
        if is_key_pressed(macroquad::prelude::KeyCode::K) {
            self.animation_duration = self.animation_duration + Duration::from_secs(1);
        }
        if is_key_pressed(macroquad::prelude::KeyCode::J) {
            self.animation_duration = self.animation_duration - Duration::from_secs(1);
            if self.animation_duration < Duration::from_secs(2) {
                self.animation_duration = Duration::from_secs(2);
            }
        }
    }

    pub fn grab_black_screen(&mut self) {
        clear_background(BLACK);
        let image = get_screen_data();
        self.black_screen = Texture2D::from_image(&image);
    }

    pub fn random_word(&self) -> String {
        self.wordlist
            .choose()
            .expect("Failed to get random word")
            .clone()
    }
}
