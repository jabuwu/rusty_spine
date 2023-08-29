use std::sync::Arc;

// tests
#[cfg(test)]
use crate::{AnimationState, AnimationStateData, Atlas, Skeleton, SkeletonData, SkeletonJson};

// doc tests
#[cfg(not(test))]
use rusty_spine::{
    AnimationState, AnimationStateData, Atlas, Skeleton, SkeletonData, SkeletonJson,
};

pub struct TestAsset {
    pub atlas_file: &'static str,
    pub atlas_data: &'static [u8],
    pub json_data: &'static [u8],
}

impl TestAsset {
    pub const fn all() -> &'static [Self; 8] {
        &[
            Self {
                atlas_file: "assets/spineboy/export/spineboy.atlas",
                atlas_data: include_bytes!("../assets/spineboy/export/spineboy.atlas"),
                json_data: include_bytes!("../assets/spineboy/export/spineboy-pro.json"),
            },
            Self {
                atlas_file: "assets/alien/export/alien.atlas",
                atlas_data: include_bytes!("../assets/alien/export/alien.atlas"),
                json_data: include_bytes!("../assets/alien/export/alien-pro.json"),
            },
            Self {
                atlas_file: "assets/coin/export/coin.atlas",
                atlas_data: include_bytes!("../assets/coin/export/coin.atlas"),
                json_data: include_bytes!("../assets/coin/export/coin-pro.json"),
            },
            Self {
                atlas_file: "assets/dragon/export/dragon.atlas",
                atlas_data: include_bytes!("../assets/dragon/export/dragon.atlas"),
                json_data: include_bytes!("../assets/dragon/export/dragon-ess.json"),
            },
            Self {
                atlas_file: "assets/goblins/export/goblins.atlas",
                atlas_data: include_bytes!("../assets/goblins/export/goblins.atlas"),
                json_data: include_bytes!("../assets/goblins/export/goblins-pro.json"),
            },
            Self {
                atlas_file: "assets/stretchyman/export/stretchyman.atlas",
                atlas_data: include_bytes!("../assets/stretchyman/export/stretchyman.atlas"),
                json_data: include_bytes!("../assets/stretchyman/export/stretchyman-pro.json"),
            },
            Self {
                atlas_file: "assets/tank/export/tank.atlas",
                atlas_data: include_bytes!("../assets/tank/export/tank.atlas"),
                json_data: include_bytes!("../assets/tank/export/tank-pro.json"),
            },
            Self {
                atlas_file: "assets/windmill/export/windmill.atlas",
                atlas_data: include_bytes!("../assets/windmill/export/windmill.atlas"),
                json_data: include_bytes!("../assets/windmill/export/windmill-ess.json"),
            },
        ]
    }

    pub const fn spineboy() -> &'static Self {
        &Self::all()[0]
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn atlas(&self) -> Atlas {
        Atlas::new(self.atlas_data, "").unwrap()
    }

    pub fn skeleton_json(&self) -> SkeletonJson {
        SkeletonJson::new(Arc::new(self.atlas()))
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn skeleton_data(&self) -> SkeletonData {
        self.skeleton_json()
            .read_skeleton_data(self.json_data)
            .unwrap()
    }

    pub fn animation_state_data(&self) -> AnimationStateData {
        AnimationStateData::new(Arc::new(self.skeleton_data()))
    }

    pub fn instance_data(&self) -> (Arc<SkeletonData>, Arc<AnimationStateData>) {
        let skeleton_data = Arc::new(self.skeleton_data());
        let animation_state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));
        (skeleton_data, animation_state_data)
    }

    pub fn instance(&self) -> (Skeleton, AnimationState) {
        let (skeleton_data, animation_state_data) = self.instance_data();
        let skeleton = Skeleton::new(skeleton_data);
        let animation_state = AnimationState::new(animation_state_data);
        (skeleton, animation_state)
    }
}

/// Ensure all the example assets load without error.
#[test]
fn load_example_assets() {
    for example_asset in TestAsset::all() {
        let atlas = Arc::new(Atlas::new(example_asset.atlas_data, "").unwrap());
        let skeleton_json = SkeletonJson::new(atlas);
        let skeleton_data = Arc::new(
            skeleton_json
                .read_skeleton_data(example_asset.json_data)
                .unwrap(),
        );
        let animation_state_data = AnimationStateData::new(skeleton_data.clone());
        let _ = Skeleton::new(skeleton_data);
        let _ = AnimationState::new(Arc::new(animation_state_data));
    }
}
