#[derive(Clone, Copy)]
pub struct Chord {
    pub l: f64,
    pub c: f64,
    pub h: f64,
}

impl Chord {
    pub fn new(l: f64, c: f64, h: f64) -> Self {
        Self { l, c, h }
    }

    pub fn render<T: From<Self>>(self) -> T {
        T::from(self)
    }

    pub fn dim(self) -> Self {
        self.with_lightness(self.l - 0.199)
    }

    pub fn ansi_dim(self) -> Self {
        self.with_lightness(self.l - 0.12)
            .with_chroma(self.c - 0.08)
    }

    pub fn ansi_bright(self) -> Self {
        self.with_lightness(self.l + 0.16)
            .with_chroma(self.c + 0.05)
    }

    pub fn ansi_desat(self) -> Self {
        self.with_chroma(self.c - 0.075)
    }

    pub fn faint(self, bg_l: f64) -> Self {
        self.with_lightness(bg_l + 0.078)
    }

    pub fn veryfaint(self, bg_l: f64) -> Self {
        self.with_lightness(bg_l + 0.046)
    }

    pub fn fgdim(self, fg_l: f64) -> Self {
        self.with_lightness(fg_l - 0.044)
    }

    pub fn interp(self, other: Self, ratio: f64) -> Self {
        Self::new(
            self.l + (other.l - self.l) * ratio,
            self.c + (other.c - self.c) * ratio,
            self.h + (other.h - self.h) * ratio,
        )
    }

    // Chroma scales proportionally with lightness to stay within sRGB gamut
    pub fn with_lightness(self, target_l: f64) -> Self {
        Self::new(target_l, self.c * target_l / self.l, self.h)
    }

    pub fn with_hue(self, target_h: f64) -> Self {
        Self::new(self.l, self.c, target_h)
    }

    pub fn with_chroma(self, target_c: f64) -> Self {
        Self::new(self.l, target_c, self.h)
    }

    pub fn rotate(self, delta_h: f64) -> Self {
        Self::new(self.l, self.c, self.h + delta_h)
    }
}
