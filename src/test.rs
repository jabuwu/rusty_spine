use std::sync::Arc;

// tests
#[cfg(test)]
use crate::{
    AnimationState, AnimationStateData, Atlas, Skeleton, SkeletonBinary, SkeletonData, SkeletonJson,
};

// doc tests
#[cfg(not(test))]
use rusty_spine::{
    AnimationState, AnimationStateData, Atlas, Skeleton, SkeletonBinary, SkeletonData, SkeletonJson,
};

pub struct TestAsset {
    pub atlas_file: &'static str,
    pub atlas_data: &'static [u8],
    pub json_data: &'static [u8],
    pub binary_data: &'static [u8],
}

impl TestAsset {
    #[must_use]
    pub const fn all() -> &'static [Self; 9] {
        &[
            Self {
                atlas_file: "assets/spineboy/export/spineboy.atlas",
                atlas_data: include_bytes!("../assets/spineboy/export/spineboy.atlas"),
                json_data: include_bytes!("../assets/spineboy/export/spineboy-pro.json"),
                binary_data: include_bytes!("../assets/spineboy/export/spineboy-pro.skel"),
            },
            Self {
                atlas_file: "assets/alien/export/alien.atlas",
                atlas_data: include_bytes!("../assets/alien/export/alien.atlas"),
                json_data: include_bytes!("../assets/alien/export/alien-pro.json"),
                binary_data: include_bytes!("../assets/alien/export/alien-pro.skel"),
            },
            Self {
                atlas_file: "assets/coin/export/coin.atlas",
                atlas_data: include_bytes!("../assets/coin/export/coin.atlas"),
                json_data: include_bytes!("../assets/coin/export/coin-pro.json"),
                binary_data: include_bytes!("../assets/coin/export/coin-pro.skel"),
            },
            Self {
                atlas_file: "assets/dragon/export/dragon.atlas",
                atlas_data: include_bytes!("../assets/dragon/export/dragon.atlas"),
                json_data: include_bytes!("../assets/dragon/export/dragon-ess.json"),
                binary_data: include_bytes!("../assets/dragon/export/dragon-ess.skel"),
            },
            Self {
                atlas_file: "assets/goblins/export/goblins.atlas",
                atlas_data: include_bytes!("../assets/goblins/export/goblins.atlas"),
                json_data: include_bytes!("../assets/goblins/export/goblins-pro.json"),
                binary_data: include_bytes!("../assets/goblins/export/goblins-pro.skel"),
            },
            Self {
                atlas_file: "assets/stretchyman/export/stretchyman.atlas",
                atlas_data: include_bytes!("../assets/stretchyman/export/stretchyman.atlas"),
                json_data: include_bytes!("../assets/stretchyman/export/stretchyman-pro.json"),
                binary_data: include_bytes!("../assets/stretchyman/export/stretchyman-pro.skel"),
            },
            Self {
                atlas_file: "assets/tank/export/tank.atlas",
                atlas_data: include_bytes!("../assets/tank/export/tank.atlas"),
                json_data: include_bytes!("../assets/tank/export/tank-pro.json"),
                binary_data: include_bytes!("../assets/tank/export/tank-pro.skel"),
            },
            Self {
                atlas_file: "assets/windmill/export/windmill.atlas",
                atlas_data: include_bytes!("../assets/windmill/export/windmill.atlas"),
                json_data: include_bytes!("../assets/windmill/export/windmill-ess.json"),
                binary_data: include_bytes!("../assets/windmill/export/windmill-ess.skel"),
            },
            Self {
                atlas_file: "assets/celestial-circus/export/celestial-circus.atlas",
                atlas_data: include_bytes!(
                    "../assets/celestial-circus/export/celestial-circus.atlas"
                ),
                json_data: include_bytes!(
                    "../assets/celestial-circus/export/celestial-circus-pro.json"
                ),
                binary_data: include_bytes!(
                    "../assets/celestial-circus/export/celestial-circus-pro.skel"
                ),
            },
        ]
    }

    #[must_use]
    pub const fn spineboy() -> &'static Self {
        &Self::all()[0]
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn atlas(&self) -> Atlas {
        Atlas::new(self.atlas_data, "").unwrap()
    }

    #[must_use]
    pub fn skeleton_json(&self) -> SkeletonJson {
        SkeletonJson::new(Arc::new(self.atlas()))
    }

    #[must_use]
    pub fn skeleton_binary(&self) -> SkeletonBinary {
        SkeletonBinary::new(Arc::new(self.atlas()))
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn skeleton_data(&self, json: bool) -> SkeletonData {
        if json {
            self.skeleton_json()
                .read_skeleton_data(self.json_data)
                .unwrap()
        } else {
            self.skeleton_binary()
                .read_skeleton_data(self.binary_data)
                .unwrap()
        }
    }

    #[must_use]
    pub fn animation_state_data(&self, json: bool) -> AnimationStateData {
        AnimationStateData::new(Arc::new(self.skeleton_data(json)))
    }

    #[must_use]
    pub fn instance_data(&self, json: bool) -> (Arc<SkeletonData>, Arc<AnimationStateData>) {
        let skeleton_data = Arc::new(self.skeleton_data(json));
        let animation_state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));
        (skeleton_data, animation_state_data)
    }

    #[must_use]
    pub fn instance(&self, json: bool) -> (Skeleton, AnimationState) {
        let (skeleton_data, animation_state_data) = self.instance_data(json);
        let skeleton = Skeleton::new(skeleton_data);
        let animation_state = AnimationState::new(animation_state_data);
        (skeleton, animation_state)
    }
}

/// Ensure all the example assets load without error.
#[test]
fn load_example_assets() {
    for example_asset in TestAsset::all() {
        _ = example_asset.instance(true);
    }
}
