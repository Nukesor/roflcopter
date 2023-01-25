use macroquad::prelude::*;

pub fn load_shaders() -> Vec<Material> {
    let lens_material = load_material(
        LENS_VERTEX_SHADER,
        LENS_FRAGMENT_SHADER,
        MaterialParams {
            uniforms: vec![("Center".to_owned(), UniformType::Float2)],
            ..Default::default()
        },
    )
    .unwrap();

    vec![lens_material]
}

const LENS_FRAGMENT_SHADER: &str = r#"#version 100
precision lowp float;

varying vec2 uv;
varying vec2 uv_screen;
varying vec2 center;

uniform sampler2D _ScreenTexture;

void main() {
    float gradient = length(uv);
    vec2 uv_zoom = (uv_screen - center) * gradient + center;

    gl_FragColor = texture2D(_ScreenTexture, uv_zoom);
}
"#;

const LENS_VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying lowp vec2 center;
varying lowp vec2 uv;
varying lowp vec2 uv_screen;

uniform mat4 Model;
uniform mat4 Projection;

uniform vec2 Center;

void main() {
    vec4 res = Projection * Model * vec4(position, 1);
    vec4 c = Projection * Model * vec4(Center, 0, 1);

    uv_screen = res.xy / 2.0 + vec2(0.5, 0.5);
    center = c.xy / 2.0 + vec2(0.5, 0.5);
    uv = texcoord;

    gl_Position = res;
}
";
