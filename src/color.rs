use crate::c::c_float;

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Color {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float,
}

impl Color {
    pub fn set_from_floats(&mut self, r: c_float, g: c_float, b: c_float, a: c_float) {
        self.r = r;
        self.g = g;
        self.b = b;
        self.a = a;
        self.clamp();
    }

    pub fn set_from_floats3(&mut self, r: c_float, g: c_float, b: c_float) {
        self.r = r;
        self.g = g;
        self.b = b;
        self.clamp();
    }

    pub fn set_from_color(&mut self, other: &Color) {
        self.r = other.r;
        self.g = other.g;
        self.b = other.b;
        self.a = other.a;
    }

    pub fn set_from_color3(&mut self, other: &Color) {
        self.r = other.r;
        self.g = other.g;
        self.b = other.b;
    }

    pub fn add_floats(&mut self, r: c_float, g: c_float, b: c_float, a: c_float) {
        self.r += r;
        self.g += g;
        self.b += b;
        self.a += a;
        self.clamp();
    }

    pub fn add_floats3(&mut self, r: c_float, g: c_float, b: c_float) {
        self.r += r;
        self.g += g;
        self.b += b;
        self.clamp();
    }

    pub fn add_color(&mut self, other: &Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
        self.a += other.a;
        self.clamp();
    }

    pub fn clamp(&mut self) {
        self.r = self.r.clamp(0., 1.);
        self.g = self.r.clamp(0., 1.);
        self.b = self.r.clamp(0., 1.);
        self.a = self.r.clamp(0., 1.);
    }
}
