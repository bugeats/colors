#[derive(Clone, Copy)]
pub struct Oklch {
    pub l: f64,
    pub c: f64,
    pub h: f64,
}

impl Oklch {
    pub fn new(l: f64, c: f64, h: f64) -> Self {
        Self { l, c, h }
    }

    pub fn to_hex(self) -> String {
        let (r, g, b) = self.to_srgb_bytes();
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    pub fn to_rgb(self) -> (u8, u8, u8) {
        self.to_srgb_bytes()
    }

    pub fn dim(self) -> Self {
        self.with_lightness(self.l - 0.199)
    }

    pub fn ansi_dim(self) -> Self {
        self.with_lightness(self.l - 0.12)
            .with_chroma(self.c - 0.032)
    }

    pub fn ansi_bright(self) -> Self {
        self.with_lightness(self.l + 0.16)
            .with_chroma(self.c + 0.02)
    }

    pub fn ansi_desat(self) -> Self {
        self.with_chroma(self.c - 0.03)
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

    // Cartesian interpolation in OKLab avoids hue discontinuity at 0/360
    pub fn interp(self, other: Self, ratio: f64) -> Self {
        let (a1, b1) = self.to_ab();
        let (a2, b2) = other.to_ab();

        let l = self.l + (other.l - self.l) * ratio;
        let a = a1 + (a2 - a1) * ratio;
        let b = b1 + (b2 - b1) * ratio;

        let c = (a * a + b * b).sqrt();
        let mut h = b.atan2(a).to_degrees();

        if h < 0.0 {
            h += 360.0;
        }

        Self::new(l, c, h)
    }

    // Chroma scales proportionally with lightness to stay within sRGB gamut
    pub fn with_lightness(self, target_l: f64) -> Self {
        Self::new(target_l, self.c * target_l / self.l, self.h)
    }

    pub fn with_hue(&self, target_h: f64) -> Self {
        Self::new(self.l, self.c, target_h)
    }

    pub fn with_chroma(&self, target_c: f64) -> Self {
        Self::new(self.l, target_c, self.h)
    }

    pub fn rotate(&self, delta_h: f64) -> Self {
        let h = (self.h + delta_h) % 360.0;
        Self::new(self.l, self.c, h)
    }

    fn to_ab(self) -> (f64, f64) {
        (
            self.c * self.h.to_radians().cos(),
            self.c * self.h.to_radians().sin(),
        )
    }

    fn to_srgb_bytes(self) -> (u8, u8, u8) {
        let (a, b) = self.to_ab();

        let l_ = self.l + 0.3963377774 * a + 0.2158037573 * b;
        let m_ = self.l - 0.1055613458 * a - 0.0638541728 * b;
        let s_ = self.l - 0.0894841775 * a - 1.2914855480 * b;

        let (l, m, s) = (l_ * l_ * l_, m_ * m_ * m_, s_ * s_ * s_);
        let lr = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
        let lg = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
        let lb = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;

        (srgb_byte(lr), srgb_byte(lg), srgb_byte(lb))
    }
}

fn srgb_byte(linear: f64) -> u8 {
    let srgb = if linear <= 0.0031308 {
        linear * 12.92
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    };

    (srgb * 255.0).round().clamp(0.0, 255.0) as u8
}
