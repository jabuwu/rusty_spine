#[macro_use]
pub mod c_interface;
pub mod c;
pub mod extension;
pub mod prelude;
pub mod sync_ptr;

mod animation;
mod animation_state;
mod animation_state_data;
mod atlas;
mod attachment;
mod bone;
mod bounding_box_attachment;
mod clipping_attachment;
mod color;
mod error;
mod mesh_attachment;
mod point_attachment;
mod region_attachment;
mod renderer_object;
mod skeleton;
mod skeleton_clipping;
mod skeleton_controller;
mod skeleton_data;
mod skeleton_json;
mod skin;
mod slot;
mod texture_region;

pub use animation::*;
pub use animation_state::*;
pub use animation_state_data::*;
pub use atlas::*;
pub use attachment::*;
pub use bone::*;
pub use bounding_box_attachment::*;
pub use clipping_attachment::*;
pub use color::*;
pub use error::*;
pub use mesh_attachment::*;
pub use point_attachment::*;
pub use region_attachment::*;
pub use renderer_object::*;
pub use skeleton::*;
pub use skeleton_clipping::*;
pub use skeleton_controller::*;
pub use skeleton_data::*;
pub use skeleton_json::*;
pub use skin::*;
pub use slot::*;
pub use texture_region::*;

#[cfg(test)]
pub mod tests;
