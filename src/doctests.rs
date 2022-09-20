use std::sync::Arc;

use rusty_spine::{
    AnimationState, AnimationStateData, Atlas, Skeleton, SkeletonData, SkeletonJson,
};

pub const TEST_SPINEBOY_ATLAS_BYTES: &'static [u8] =
    include_bytes!("../assets/spineboy/export/spineboy.atlas");
pub const TEST_SPINEBOY_JSON_BYTES: &'static [u8] =
    include_bytes!("../assets/spineboy/export/spineboy-pro.json");

pub fn test_spineboy_atlas() -> Atlas {
    Atlas::new(TEST_SPINEBOY_ATLAS_BYTES, "").unwrap()
}

pub fn test_spineboy_skeleton_json() -> SkeletonJson {
    SkeletonJson::new(Arc::new(test_spineboy_atlas()))
}

pub fn test_spineboy_instance_data() -> (Arc<SkeletonData>, Arc<AnimationStateData>) {
    let skeleton_data = Arc::new(
        test_spineboy_skeleton_json()
            .read_skeleton_data(TEST_SPINEBOY_JSON_BYTES)
            .unwrap(),
    );
    let animation_state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));
    (skeleton_data, animation_state_data)
}

pub fn test_spineboy_instance() -> (Skeleton, AnimationState) {
    let (skeleton_data, animation_state_data) = test_spineboy_instance_data();
    let skeleton = Skeleton::new(skeleton_data).unwrap();
    let animation_state = AnimationState::new(animation_state_data);
    (skeleton, animation_state)
}
