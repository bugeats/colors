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
            interval: Vector3::new(0.5, 0.0, 0.0),
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
            point: self.point - Vector3::new(0.199, 0.0, 0.0),
            ..self
        }
    }

    pub fn light(self) -> Self {
        Self {
            point: self.point + Vector3::new(0.16, 0.0, 0.0),
            ..self
        }
    }

    pub fn rotate(self, delta: f64) -> Self {
        Self {
            point: self.point + Vector3::new(0.0, 0.0, delta),
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
        let bottom_l = self.point[0] - self.interval[0] / 2.0;

        Self {
            point: Vector3::new(bottom_l + 0.078, self.point[1], self.point[2]),
            interval: Vector3::zeros(),
        }
    }

    pub fn tint(self) -> Self {
        let bottom_l = self.point[0] - self.interval[0] / 2.0;

        Self {
            point: Vector3::new(bottom_l + 0.046, self.point[1], self.point[2]),
            interval: Vector3::zeros(),
        }
    }

    pub fn soften(self) -> Self {
        Self {
            interval: self.interval - Vector3::new(0.09, 0.0, 0.0),
            ..self
        }
    }
}
