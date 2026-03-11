use std::fmt;

use palette::{Clamp, FromColor, Oklch, Srgb};

use crate::chord::Color;

const MAX_C: f64 = 0.4;
const MAX_H: f64 = 360.0;

#[derive(Clone, Copy)]
pub struct ThemeRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Color> for ThemeRgb {
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

impl From<ThemeRgb> for anstyle::Color {
    fn from(rgb: ThemeRgb) -> Self {
        anstyle::RgbColor(rgb.r, rgb.g, rgb.b).into()
    }
}

impl fmt::Display for ThemeRgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}
