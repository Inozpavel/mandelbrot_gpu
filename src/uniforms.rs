use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use std::fmt::{Display, Formatter};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Uniforms {
    pub center: [f32; 4],        // 2 points, 16
    pub initial_value: [f32; 4], // 2 points, 16
    pub max_iter: u32,           // 4
    pub zoom: f32,               // 4
    pub rgb_green: f32,          // 4
    pub rgb_blue: f32,           // 4
    pub color_scheme: u32,       // 4
    pub hsv_saturation: f32,     // 4
    pub hsv_brightness: f32,     // 4
    pub show_axis: u32,          // 4
    pub escape_threshold: f32,   // 4
    pub fractal_type: u32,       // 4
    pub pow: u32,                // 4
    pub pad: [u8; 4],
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FractalColorScheme: u32 {
        const RGB = 1;
        const HSV = 2;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FractalType: u32 {
        const MANDELBROT = 1;
        const JULIA = 2;
    }
}

impl Display for FractalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut parts = vec![];

        if self.contains(Self::JULIA) {
            parts.push("Джулиа");
        }
        if self.contains(Self::MANDELBROT) {
            parts.push("Мандельброт");
        }

        if parts.is_empty() {
            write!(f, "(none)")
        } else {
            write!(f, "{}", parts.join(" | "))
        }
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
