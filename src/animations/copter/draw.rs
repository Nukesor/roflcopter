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

/// Generate the helicopter texture, depending on the directions.
pub fn generate_copter_texture(
    state: &State,
    copter_direction: Side,
    rotor_direction: Side,
) -> Texture2D {
    let text = get_ascii_art(&rotor_direction, &copter_direction);

    texture_from_text(state, &text, state.font_size, None)
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
