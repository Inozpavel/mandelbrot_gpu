use crate::uniforms::{FractalColorScheme, FractalType};

pub struct UserSettings {
    pub max_iter: u32,
    pub center_x: f32,
    pub center_y: f32,
    pub initial_value_x: f32,
    pub initial_value_y: f32,
    pub zoom: f32,
    pub color_scheme: FractalColorScheme,
    pub rgb_green: f32,
    pub rgb_blue: f32,
    pub hsv_saturation: f32,
    pub hsv_brightness: f32,
    pub show_settings: bool,
    pub show_axis: bool,
    pub escape_threshold: f32,
    pub fractal_type: FractalType,
}

impl UserSettings {
    pub fn new() -> Self {
        Self {
            max_iter: 125,
            zoom: 0.75,
            center_x: -0.33,
            center_y: 0.0,
            initial_value_x: 0.0,
            initial_value_y: 0.0,
            color_scheme: FractalColorScheme::HSV,
            rgb_green: 0.8,
            rgb_blue: 0.8,
            hsv_saturation: 1.0,
            hsv_brightness: 1.0,
            show_settings: true,
            show_axis: false,
            escape_threshold: 4.0,
            fractal_type: FractalType::MANDELBROT,
        }
    }
}
