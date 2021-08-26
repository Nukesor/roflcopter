use macroquad::prelude::*;

use super::CopterImages;
use crate::animations::helper::Direction;

/// Lower level helicopter drawing call.
/// This is a simple wrapper around some of macroquads drawing logic.
pub fn draw_copter(
    images: &CopterImages,
    copter_direction: &Direction,
    rotor_direction: &Direction,
    x: f32,
    y: f32,
    rotation: f32,
) {
    let texture = images.get_for_directions(copter_direction, rotor_direction);
    draw_texture_ex(
        texture,
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
