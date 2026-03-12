use nalgebra::Vector3;

pub type Color = Vector3<f64>;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Chord {
    pub point: Color,
    pub interval: Color,
}

impl From<Color> for Chord {
    fn from(point: Color) -> Self {
        Self {
            point,
            interval: Vector3::x() * 0.5,
        }
    }
}

impl Chord {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }

    pub fn top(self) -> Color {
        self.point + self.interval / 2.0
    }

    pub fn bottom(self) -> Color {
        self.point - self.interval / 2.0
    }

    pub fn middle(self) -> Color {
        self.point
    }

    pub fn get_lit(&self) -> f64 {
        self.point[0]
    }

    pub fn set_lit(self, target_lit: f64) -> Self {
        Self {
            point: Vector3::new(target_lit, self.point[1], self.point[2]),
            ..self
        }
    }

    pub fn set_sat(self, target_sat: f64) -> Self {
        Self {
            point: Vector3::new(self.point[0], target_sat, self.point[2]),
            ..self
        }
    }

    pub fn mk_blue(self) -> Self {
        Self {
            point: Vector3::new(self.point[0], self.point[1], 0.7),
            ..self
        }
    }

    pub fn mk_green(self) -> Self {
        self.mk_red().rotate(5.0 / 24.0)
    }

    pub fn sat(&self) -> f64 {
        self.point[1]
    }

    pub fn lit(&self) -> f64 {
        self.point[0]
    }

    pub fn mk_saturated(self) -> Self {
        self.set_sat(self.sat() + 0.14)
    }

    pub fn mk_bamp(self, seed: u64) -> Self {
        const AMP: f64 = 0.02;

        let shift = Color::new(
            noise(seed, 0) * AMP,
            noise(seed, 1) * AMP,
            noise(seed, 2) * AMP,
        );

        Self {
            point: self.point + shift,
            ..self
        }
    }

    pub fn mk_red(self) -> Self {
        Self {
            point: Vector3::new(self.point[0], self.point[1], 0.1),
            ..self
        }
    }

    pub fn browntown(self) -> Self {
        let intr = self.interval + Vector3::new(-0.2, -0.1, 0.1);

        self.mk_red()
            .rotate(1.0 / 8.0)
            .set_sat(self.sat() - 0.13)
            .set_lit(self.lit() - 0.09)
            .set_interval(intr)
    }

    pub fn inverted(self) -> Self {
        Self {
            point: self.bottom(),
            interval: -self.interval,
        }
    }

    pub fn pin_bottom(self, other: &Chord) -> Self {
        Self {
            interval: 2.0 * (self.point - other.bottom()),
            ..self
        }
    }

    pub fn pushback(self) -> Self {
        self.set_lit(self.lit() * 3.0 / 4.0)
    }

    pub fn pushup(self) -> Self {
        self.set_lit(self.lit() * 5.0 / 4.0)
    }

    pub fn set_hue(self, target_hue: f64) -> Self {
        Self {
            point: Vector3::new(self.point[0], self.point[1], target_hue),
            ..self
        }
    }

    pub fn set_interval(self, interval: Vector3<f64>) -> Self {
        Self {
            interval: interval.into(),
            ..self
        }
    }

    pub fn active(self) -> Self {
        Self {
            point: Vector3::new(self.point[0] + 0.12, self.point[1] + 0.2, self.point[2]),
            ..self
        }
    }

    pub fn rotate(self, delta: f64) -> Self {
        Self {
            point: self.point + Vector3::z() * delta,
            ..self
        }
    }

    pub fn desaturated(self) -> Self {
        Self {
            point: Vector3::new(self.point[0], 0.01, self.point[2]),
            ..self
        }
    }

    pub fn faint(self) -> Self {
        Self {
            point: Vector3::new(0.27, self.point[1], self.point[2]),
            ..self
        }
    }
}

/// Deterministic hash noise: maps (seed, dimension) to [-1, 1].
fn noise(seed: u64, dim: u64) -> f64 {
    let mut z = seed.wrapping_add(dim.wrapping_mul(0x9e3779b97f4a7c15));

    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^= z >> 31;

    (z as f64) / (u64::MAX as f64) * 2.0 - 1.0
}
