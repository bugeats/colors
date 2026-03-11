use std::fmt;

use palette::{Clamp, FromColor, Oklab, Oklch, Srgb};

use crate::chord::Color;

const MAX_C: f64 = 0.4;
const MAX_H: f64 = 360.0;

pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Color> for Rgb {
    fn from(color: Color) -> Self {
        let oklch = Oklch::new(
            color[0],
            (color[1] * MAX_C).max(0.0),
            color[2].rem_euclid(1.0) * MAX_H,
        );

        let srgb: Srgb<u8> = Srgb::from_color(oklch).clamp().into_format();

        Self {
            r: srgb.red,
            g: srgb.green,
            b: srgb.blue,
        }
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}
