use nalgebra::Vector3;

pub type Color = Vector3<f64>;

#[derive(Clone, Copy)]
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

    pub fn set_hue(self, target_hue: f64) -> Self {
        Self {
            point: Vector3::new(self.point[0], self.point[1], target_hue),
            ..self
        }
    }

    pub fn set_interval<T: Into<Vector3<f64>>>(self, interval: T) -> Self {
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
