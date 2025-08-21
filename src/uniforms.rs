use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use std::fmt::Display;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Uniforms {
    pub center: [f32; 4],  // 2 points, 16
    pub initial_value: [f32; 4], // 2 points, 16
    pub max_iter: u32,     // 4
    pub zoom: f32,         // 4
    pub rgb_green: f32,    // 4
    pub rgb_blue: f32,     // 4
    pub color_scheme: u32, // 4
    pub hsv_saturation: f32, // 4
    pub hsv_brightness: f32, // 4
    pub show_axis: u32, // 4
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FractalColorScheme: u32 {
        const RGB = 1;
        const HSV = 2;
    }
}

impl Display for FractalColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = vec![];

        if self.contains(Self::RGB) {
            parts.push("RGB");
        }
        if self.contains(Self::HSV) {
            parts.push("HSV");
        }

        if parts.is_empty() {
            write!(f, "(none)")
        } else {
            write!(f, "{}", parts.join(" | "))
        }
    }
}
