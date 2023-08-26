use std::ops::{Mul, MulAssign};

use crate::c::c_float;

/// RGBA F32 color that is byte-compatible with the Spine runtime.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Color {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float,
}

impl Color {
    #[must_use]
    pub const fn new_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn set_r(&mut self, r: c_float) -> &mut Self {
        self.r = r;
        self
    }

    pub fn set_g(&mut self, g: c_float) -> &mut Self {
        self.g = g;
        self
    }

    pub fn set_b(&mut self, b: c_float) -> &mut Self {
        self.b = b;
        self
    }

    pub fn set_a(&mut self, a: c_float) -> &mut Self {
        self.a = a;
        self
    }

    pub fn set_from_floats(&mut self, r: c_float, g: c_float, b: c_float, a: c_float) -> &mut Self {
        self.r = r;
        self.g = g;
        self.b = b;
        self.a = a;
        self.clamp();
        self
    }

    pub fn set_from_floats3(&mut self, r: c_float, g: c_float, b: c_float) -> &mut Self {
        self.r = r;
        self.g = g;
        self.b = b;
        self.clamp();
        self
    }

    pub fn set_from_color(&mut self, other: &Color) -> &mut Self {
        self.r = other.r;
        self.g = other.g;
        self.b = other.b;
        self.a = other.a;
        self
    }

    pub fn set_from_color3(&mut self, other: &Color) -> &mut Self {
        self.r = other.r;
        self.g = other.g;
        self.b = other.b;
        self
    }

    pub fn add_floats(&mut self, r: c_float, g: c_float, b: c_float, a: c_float) -> &mut Self {
        self.r += r;
        self.g += g;
        self.b += b;
        self.a += a;
        self.clamp();
        self
    }

    pub fn add_floats3(&mut self, r: c_float, g: c_float, b: c_float) -> &mut Self {
        self.r += r;
        self.g += g;
        self.b += b;
        self.clamp();
        self
    }

    pub fn add_color(&mut self, other: &Color) -> &mut Self {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
        self.a += other.a;
        self.clamp();
        self
    }

    pub fn clamp(&mut self) -> &mut Self {
        self.r = self.r.clamp(0., 1.);
        self.g = self.r.clamp(0., 1.);
        self.b = self.r.clamp(0., 1.);
        self.a = self.r.clamp(0., 1.);
        self
    }

    pub fn premultiply_alpha(&mut self) {
        self.r *= self.a;
        self.g *= self.a;
        self.b *= self.a;
    }

    #[must_use]
    pub fn linear_to_nonlinear(&self) -> Color {
        Color {
            r: linear_to_nonlinear(self.r),
            g: linear_to_nonlinear(self.g),
            b: linear_to_nonlinear(self.b),
            a: self.a,
        }
    }

    #[must_use]
    pub fn nonlinear_to_linear(&self) -> Color {
        Color {
            r: nonlinear_to_linear(self.r),
            g: nonlinear_to_linear(self.g),
            b: nonlinear_to_linear(self.b),
            a: self.a,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Color) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

impl From<[f32; 4]> for Color {
    fn from(value: [f32; 4]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}

fn linear_to_nonlinear(x: f32) -> f32 {
    if x <= 0.0 {
        x
    } else if x <= 0.0031308 {
        x * 12.92
    } else {
        (1.055 * x.powf(1.0 / 2.4)) - 0.055
    }
}

fn nonlinear_to_linear(x: f32) -> f32 {
    if x <= 0.0 {
        x
    } else if x <= 0.04045 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}
