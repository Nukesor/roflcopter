use macroquad::color::Color;

pub fn create_colors() -> Vec<Color> {
    vec![
        // Red
        Color::from_rgba(255, 0, 0, 255),
        // Orange
        Color::from_rgba(255, 120, 0, 255),
        // Yellow
        Color::from_rgba(255, 238, 0, 255),
        // Bright green
        Color::from_rgba(153, 255, 0, 255),
        // Green
        Color::from_rgba(35, 255, 0, 255),
        // Turquoise
        Color::from_rgba(255, 198, 0, 255),
        // Bright blue
        Color::from_rgba(0, 194, 255, 255),
        // Blue
        Color::from_rgba(0, 75, 255, 255),
        //// Dark Blue
        //Color::from_rgba(42, 0, 255, 255),
        // Purple
        Color::from_rgba(157, 0, 255, 255),
        // Pink
        Color::from_rgba(255, 0, 232, 255),
    ]
}
