use std::sync::Arc;

use rusty_spine::{controller::*, *};

fn main() {
    let atlas_path = "assets/spineboy/export/spineboy.atlas";
    let json_path = "assets/spineboy/export/spineboy-pro.json";
    let atlas = Arc::new(Atlas::new_from_file(atlas_path).unwrap());
    let skeleton_json = SkeletonJson::new(atlas);
    let skeleton_data = Arc::new(skeleton_json.read_skeleton_data_file(json_path).unwrap());
    let animation_state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));
    let mut skeleton_controller =
        SkeletonController::new(skeleton_data.clone(), animation_state_data);
    skeleton_controller.update(0.016);
    let renderables = skeleton_controller.renderables();
    println!("Skeleton:");
    println!();
    println!("  Atlas: {atlas_path}");
    println!("  JSON: {json_path}");
    println!("  Version: {}", skeleton_data.version().unwrap_or("??"));
    println!("  Hash: {}", skeleton_data.hash());
    println!();
    println!("Renderables:");
    println!();
    for renderable in &renderables {
        let slot = skeleton_controller
            .skeleton
            .slot_at_index(renderable.slot_index)
            .unwrap();
        println!("  {}", slot.data().name());
        println!("    {} Vertices / UVs", renderable.vertices.len());
        println!("    {} Indices", renderable.indices.len());
        println!("    {:?} Blend Mode", renderable.blend_mode);
        println!("    {:?}", renderable.color);
        println!(
            "    {}",
            if renderable.premultiplied_alpha {
                "Premultiplied Alpha"
            } else {
                "No Premultiplied Alpha"
            }
        );
        println!();
    }
}
