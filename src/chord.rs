use nalgebra::Vector3;

pub type Color = Vector3<f64>;

#[derive(Clone, Copy)]
pub struct Chord {
    point: Color,
    interval: Color,
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

    pub fn set_interval(self, interval: [f64; 3]) -> Self {
        Self {
            interval: Vector3::from(interval),
            ..self
        }
    }

    pub fn dim(self) -> Self {
        Self {
            point: self.point - Vector3::x() * 0.199,
            ..self
        }
    }

    pub fn light(self) -> Self {
        Self {
            point: self.point + Vector3::x() * 0.16,
            ..self
        }
    }

    pub fn rotate(self, delta: f64) -> Self {
        Self {
            point: self.point + Vector3::z() * delta,
            ..self
        }
    }

    pub fn ansi(self) -> Self {
        Self {
            point: Vector3::new(
                self.point[0],
                (self.point[1] - 0.075).max(0.0),
                self.point[2],
            ),
            ..self
        }
    }

    pub fn faint(self) -> Self {
        Self {
            point: self.point + Vector3::x() * (0.078 - self.interval[0] / 2.0),
            interval: Vector3::zeros(),
        }
    }

    pub fn tint(self) -> Self {
        Self {
            point: self.point + Vector3::x() * (0.046 - self.interval[0] / 2.0),
            interval: Vector3::zeros(),
        }
    }

    pub fn soften(self) -> Self {
        Self {
            interval: self.interval - Vector3::x() * 0.09,
            ..self
        }
    }
}
