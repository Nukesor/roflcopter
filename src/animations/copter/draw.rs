use macroquad::prelude::*;

use super::{CopterImages, Shot};
use crate::{helper::*, state::State};

/// Lower level helicopter drawing call.
/// This is a simple wrapper around some of macroquads drawing logic.
pub fn draw_copter(
    images: &CopterImages,
    copter_direction: &Side,
    rotor_direction: &Side,
    x: f32,
    y: f32,
    rotation: f32,
) {
    let texture = images.get_for_directions(copter_direction, rotor_direction);
    draw_texture_ex(
        *texture,
        x,
        y,
        Color::from_rgba(255, 255, 255, 255),
        DrawTextureParams {
            rotation,
            flip_y: true,
            ..Default::default()
        },
    )
}

/// Draw the actual helicopter one character at a time.
/// Once this is done, we create a image from that data.
///
/// That raw pre-compiled image can then be re-used for the rest of the animation.
pub fn draw_raw_copter(state: &State, copter_direction: Side, rotor_direction: Side) -> Texture2D {
    clear_background(Color::from_rgba(0, 0, 0, 0));
    let mut x: f32;
    let mut y: f32 = 0.0;
    let art = get_ascii_art(&rotor_direction, &copter_direction);

    // Remember the amount of lines and the max character width.
    // We need this to calculate the rectangle that should be extracted from the
    // full screen later on.
    let mut max_y = 0.0;
    let mut max_x = 0.0;

    //println!("timer: {:?}, duration: {:?}", timer, duration);
    //println!("offset: {}", offset);
    //println!("current_rotation: {}", offset);
    for line in art.lines() {
        x = 0.0;
        for character in line.chars() {
            draw_text_ex(
                &character.to_string(),
                x,
                y,
                TextParams {
                    font: state.font,
                    font_size: state.font_size,
                    font_scale: 1.0,
                    ..Default::default()
                },
            );
            x += state.font_dimensions.width;
        }
        // Move the draw position to the next
        y += state.font_dimensions.height;

        if x > max_x {
            max_x = x;
        }

        if y > max_y {
            max_y = y;
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

    // Create a texture from our roflcopter image.
    Texture2D::from_image(&image)
}

/// Get the correct ascii art, depending on the copter direction and rotor orientation.
fn get_ascii_art(rotor_direction: &Side, copter_direction: &Side) -> String {
    match copter_direction {
        Side::Right => match rotor_direction {
            Side::Left => "
   LFOR:LFOR:
         ___^___ _
 L    __/      [] \\
LOL===__           \\
 L      \\___ ___ ___]
            I   I
          ----------/"
                .to_string(),
            Side::Right => "
            :ROFL:ROFL
         ___^___ _
L L   __/      [] \\
 O ===__           \\
L L     \\___ ___ ___]
            I   I
          ----------/"
                .to_string(),
        },
        Side::Left => match rotor_direction {
            Side::Left => "
ROFL:ROFL:
    _ ___^___
   / []      \\__    L
  /           __===LOL
 [___ ___ ___/      L
      I   I
 \\----------          "
                .to_string(),
            Side::Right => "
         :LFOR:LFOR
    _ ___^___
   / []      \\__   L L
  /           __=== O
 [___ ___ ___/     L L
      I   I
 \\----------          "
                .to_string(),
        },
    }
}

/// Lower level helicopter drawing call.
/// This is a simple wrapper around some of macroquads drawing logic.
pub fn draw_shot(images: &CopterImages, shot: &Shot) {
    let flip_x = match shot.direction {
        Side::Left => true,
        Side::Right => false,
    };

    draw_texture_ex(
        images.shot,
        shot.position.x,
        shot.position.y,
        Color::from_rgba(255, 255, 255, 255),
        DrawTextureParams {
            rotation: shot.angle,
            flip_y: true,
            flip_x,
            ..Default::default()
        },
    )
}
