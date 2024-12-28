use std::cmp::{min, max};

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Hsl {
    pub h: f32,  // 0-360
    pub s: f32,  // 0-100
    pub l: f32,  // 0-100
}

impl Rgb {
    fn to_hsl(&self) -> Hsl {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max_val = max_f32(r, max_f32(g, b));
        let min_val = min_f32(r, min_f32(g, b));
        let delta = max_val - min_val;

        let mut h = 0.0;
        let mut s = 0.0;
        let l = (max_val + min_val) / 2.0;

        if delta != 0.0 {
            s = if l < 0.5 {
                delta / (max_val + min_val)
            } else {
                delta / (2.0 - max_val - min_val)
            };

            h = match max_val {
                x if x == r => (g - b) / delta + (if g < b { 6.0 } else { 0.0 }),
                x if x == g => (b - r) / delta + 2.0,
                _ => (r - g) / delta + 4.0,
            };

            h /= 6.0;
        }

        Hsl {
            h: h * 360.0,
            s: s * 100.0,
            l: l * 100.0,
        }
    }
}

impl Hsl {
    fn to_rgb(&self) -> Rgb {
        let h = self.h / 360.0;
        let s = self.s / 100.0;
        let l = self.l / 100.0;

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;

        let r = hue_to_rgb(p, q, h + 1.0/3.0);
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - 1.0/3.0);

        Rgb {
            r: (r * 255.0).round() as u8,
            g: (g * 255.0).round() as u8,
            b: (b * 255.0).round() as u8,
        }
    }
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 { t += 1.0 }
    if t > 1.0 { t -= 1.0 }
    
    if t < 1.0/6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0/2.0 {
        return q;
    }
    if t < 2.0/3.0 {
        return p + (q - p) * (2.0/3.0 - t) * 6.0;
    }
    p
}

fn max_f32(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

fn min_f32(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

// Image enhancement functions
pub trait ImageEnhancement {
    fn adjust_brightness(&mut self, amount: f32);
    fn adjust_contrast(&mut self, amount: f32);
    fn adjust_saturation(&mut self, amount: f32);
    fn adjust_shadows(&mut self, amount: f32);
    fn adjust_highlights(&mut self, amount: f32);
}

impl ImageEnhancement for Rgb {
    fn adjust_brightness(&mut self, amount: f32) {
        let mut hsl = self.to_hsl();
        hsl.l = (hsl.l + amount).clamp(0.0, 100.0);
        *self = hsl.to_rgb();
    }

    fn adjust_contrast(&mut self, amount: f32) {
        let mut hsl = self.to_hsl();
        let mid = 50.0;
        hsl.l = mid + (hsl.l - mid) * amount;
        hsl.l = hsl.l.clamp(0.0, 100.0);
        *self = hsl.to_rgb();
    }

    fn adjust_saturation(&mut self, amount: f32) {
        let mut hsl = self.to_hsl();
        hsl.s = (hsl.s * amount).clamp(0.0, 100.0);
        *self = hsl.to_rgb();
    }

    fn adjust_shadows(&mut self, amount: f32) {
        let mut hsl = self.to_hsl();
        if hsl.l < 50.0 {
            hsl.l = (hsl.l + amount).clamp(0.0, 50.0);
            *self = hsl.to_rgb();
        }
    }

    fn adjust_highlights(&mut self, amount: f32) {
        let mut hsl = self.to_hsl();
        if hsl.l > 50.0 {
            hsl.l = (hsl.l - amount).clamp(50.0, 100.0);
            *self = hsl.to_rgb();
        }
    }
}