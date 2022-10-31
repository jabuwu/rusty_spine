mod combined;
mod simple;

pub use combined::*;
pub use simple::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    SRGB,
    Linear,
}
