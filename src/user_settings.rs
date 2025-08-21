use crate::uniforms::FractalColorScheme;

pub struct UserSettings {
    pub max_iter: u32,
    pub center_x: f32,
    pub center_y: f32,
    pub zoom: f32,
    pub color_scheme: FractalColorScheme,
    pub rgb_green: f32,
    pub rgb_blue: f32,
    pub hsv_saturation: f32,
    pub hsv_brightness: f32,
}

impl UserSettings {
    pub fn new() -> Self {
        Self {
            max_iter: 100,
            zoom: 1.0,
            center_x: -0.33,
            center_y: 0.0,
            color_scheme: FractalColorScheme::HSV,
            rgb_green: 0.8,
            rgb_blue: 0.8,
            hsv_saturation: 1.0,
            hsv_brightness: 1.0,
        }
    }
}
