use std::fmt;

use palette::{Clamp, FromColor, Oklch, Srgb};

use crate::chord::Color;

const MAX_C: f64 = 0.4;
const MAX_H: f64 = 360.0;

fn to_srgb_bytes(color: Color) -> (u8, u8, u8) {
    let oklch = Oklch::new(
        color[0],
        (color[1] * MAX_C).max(0.0),
        color[2].rem_euclid(1.0) * MAX_H,
    );
    let rgb: Srgb<u8> = Srgb::from_color(oklch).clamp().into_format();
    (rgb.red, rgb.green, rgb.blue)
}

pub struct OklchHex {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Color> for OklchHex {
    fn from(color: Color) -> Self {
        let (r, g, b) = to_srgb_bytes(color);
        Self { r, g, b }
    }
}

impl fmt::Display for OklchHex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

pub struct OklchRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Color> for OklchRgb {
    fn from(color: Color) -> Self {
        let (r, g, b) = to_srgb_bytes(color);
        Self { r, g, b }
    }
}

impl fmt::Display for OklchRgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}
