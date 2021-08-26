use macroquad::prelude::*;

use crate::state::State;

pub fn draw_shader(state: &State) {
    let lens_center = mouse_position();
    let lens_material = state.shaders[0];
    lens_material.set_uniform("Center", lens_center);

    gl_use_material(lens_material);
    draw_circle(lens_center.0, lens_center.1, 400.0, RED);
    gl_use_default_material();
}
