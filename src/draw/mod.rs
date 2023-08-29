//! Helpers types for drawing Spine skeletons.
//!
//! Drawers generate mesh information ready to be used in graphics libraries and game engines.
//!
//! Two implementations are currently provided:
//! - [`SimpleDrawer`]
//! - [`CombinedDrawer`]

mod combined;
mod simple;

pub use combined::*;
pub use simple::*;

/// Cull direction to use with helper draw functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullDirection {
    Clockwise,
    CounterClockwise,
}

/// Color space to use with helper draw functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    SRGB,
    Linear,
}
