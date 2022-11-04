//! Spine runtime for Rust (and wasm!) transpiled from the official C Runtime. Supports Spine 4.1.
//!
//! To load a [`Skeleton`], see [`SkeletonJson`] or [`SkeletonBinary`].
//!
//! To set automatic mix durations (crossfading) between animations, see
//! [`AnimationStateData`].
//!
//! To find and manage bones, see [`Bone`].
//!
//! To receive animation events, see [`Event`].

#[macro_use]
pub mod c_interface;
pub mod c;
pub mod extension;

#[cfg(feature = "draw_functions")]
pub mod controller;
#[cfg(feature = "draw_functions")]
pub mod draw;

#[cfg(feature = "egui_debugger")]
pub mod debugger;

mod animation;
mod animation_state;
mod animation_state_data;
#[path = "atlas.rs"]
mod atlas_mod;
mod attachment;
mod bone;
mod bounding_box_attachment;
mod clipping_attachment;
mod color;
mod error;
mod event;
mod mesh_attachment;
mod path_attachment;
mod point_attachment;
mod region_attachment;
mod renderer_object;
mod skeleton;
mod skeleton_binary;
mod skeleton_clipping;
mod skeleton_data;
mod skeleton_json;
mod skin;
mod slot;
mod texture_region;

pub use animation::*;
pub use animation_state::*;
pub use animation_state_data::*;
pub use atlas_mod::{atlas, Atlas};
pub use attachment::*;
pub use bone::*;
pub use bounding_box_attachment::*;
pub use clipping_attachment::*;
pub use color::*;
pub use error::*;
pub use event::*;
pub use mesh_attachment::*;
pub use path_attachment::*;
pub use point_attachment::*;
pub use region_attachment::*;
pub use renderer_object::*;
pub use skeleton::*;
pub use skeleton_binary::*;
pub use skeleton_clipping::*;
pub use skeleton_data::*;
pub use skeleton_json::*;
pub use skin::*;
pub use slot::*;
pub use texture_region::*;

#[cfg(test)]
pub mod tests;
