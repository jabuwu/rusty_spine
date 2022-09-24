use std::{
    ffi::{CStr, CString},
    path::Path,
    sync::Arc,
};

use crate::{
    c::{
        spSkeletonJson, spSkeletonJson_create, spSkeletonJson_dispose,
        spSkeletonJson_readSkeletonData, spSkeletonJson_readSkeletonDataFile,
    },
    c_interface::SyncPtr,
    error::Error,
    skeleton_data::SkeletonData,
    Atlas,
};

/// A loader for Spine json files.
#[derive(Debug)]
pub struct SkeletonJson {
    c_skeleton_json: SyncPtr<spSkeletonJson>,
    owns_memory: bool,
    atlas: Option<Arc<Atlas>>,
}

impl SkeletonJson {
    /// Create a new JSON loader using the default atlas attachment loader.
    ///
    /// ```
    /// use std::sync::Arc;
    /// use rusty_spine::{AnimationState, AnimationStateData, Atlas, Error, Skeleton, SkeletonJson};
    ///
    /// fn load_skeleton() -> Result<(Skeleton, AnimationState), Error> {
    ///     let atlas = Arc::new(Atlas::new_from_file("spineboy.atlas")?);
    ///     let skeleton_json = SkeletonJson::new(atlas);
    ///     let skeleton_data = Arc::new(skeleton_json.read_skeleton_data_file("spineboy.json")?);
    ///     let animation_state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));
    ///     let skeleton = Skeleton::new(skeleton_data);
    ///     let animation_state = AnimationState::new(animation_state_data);
    ///     Ok((skeleton, animation_state))
    /// }
    /// ```
    pub fn new(atlas: Arc<Atlas>) -> Self {
        let c_skeleton_json = unsafe { spSkeletonJson_create(atlas.c_ptr()) };
        Self {
            c_skeleton_json: SyncPtr(c_skeleton_json),
            owns_memory: true,
            atlas: Some(atlas),
        }
    }

    pub fn read_skeleton_data(&self, json: &[u8]) -> Result<SkeletonData, Error> {
        let c_json = CString::new(json)?;
        let c_skeleton_data =
            unsafe { spSkeletonJson_readSkeletonData(self.c_skeleton_json.0, c_json.as_ptr()) };
        if !c_skeleton_data.is_null() {
            Ok(SkeletonData::new(c_skeleton_data, self.atlas.clone()))
        } else {
            let c_error = unsafe { CStr::from_ptr((*self.c_skeleton_json.0).error) };
            Err(Error::new_from_spine(c_error.to_str().unwrap()))
        }
    }

    pub fn read_skeleton_data_file<P: AsRef<Path>>(&self, path: P) -> Result<SkeletonData, Error> {
        let c_path = CString::new(path.as_ref().to_str().unwrap())?;
        let c_skeleton_data =
            unsafe { spSkeletonJson_readSkeletonDataFile(self.c_skeleton_json.0, c_path.as_ptr()) };
        if !c_skeleton_data.is_null() {
            Ok(SkeletonData::new(c_skeleton_data, self.atlas.clone()))
        } else {
            let c_error = unsafe { CStr::from_ptr((*self.c_skeleton_json.0).error) };
            Err(Error::new_from_spine(c_error.to_str().unwrap()))
        }
    }

    c_accessor_mut!(scale, set_scale, scale, f32);
    c_ptr!(c_skeleton_json, spSkeletonJson);
}

impl Drop for SkeletonJson {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                spSkeletonJson_dispose(self.c_skeleton_json.0);
            }
        }
    }
}
