pub struct UserSettings {
    pub iterations_limit: u32,
    pub center_x: f32,
    pub center_y: f32,
}

impl UserSettings {
    pub fn new() -> Self {
        Self {
            iterations_limit: 100,
            center_x: -0.33,
            center_y: 0.0,
        }
    }
}
