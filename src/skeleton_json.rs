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
    c_interface::{from_c_str, SyncPtr},
    error::SpineError,
    skeleton_data::SkeletonData,
    Atlas,
};

/// A loader for Spine json files.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#SkeletonJson)
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
    /// use rusty_spine::{AnimationState, AnimationStateData, Atlas, SpineError, Skeleton, SkeletonJson};
    ///
    /// fn load_skeleton() -> Result<(Skeleton, AnimationState), SpineError> {
    ///     let atlas = Arc::new(Atlas::new_from_file("spineboy.atlas")?);
    ///     let skeleton_json = SkeletonJson::new(atlas);
    ///     let skeleton_data = Arc::new(skeleton_json.read_skeleton_data_file("spineboy.json")?);
    ///     let animation_state_data = AnimationStateData::new(skeleton_data.clone());
    ///     // ... prepare animation state (see AnimationStateData docs) ...
    ///     let skeleton = Skeleton::new(skeleton_data);
    ///     let animation_state = AnimationState::new(Arc::new(animation_state_data));
    ///     Ok((skeleton, animation_state))
    /// }
    /// ```
    #[must_use]
    pub fn new(atlas: Arc<Atlas>) -> Self {
        let c_skeleton_json = unsafe { spSkeletonJson_create(atlas.c_ptr()) };
        Self {
            c_skeleton_json: SyncPtr(c_skeleton_json),
            owns_memory: true,
            atlas: Some(atlas),
        }
    }

    /// Read the Spine skeleton json data in-memory. See [`SkeletonJson::new`] for a full example.
    ///
    /// # Errors
    ///
    /// Returns [`SpineError::ParsingFailed`] if parsing of the json data failed.
    pub fn read_skeleton_data(&self, json: &[u8]) -> Result<SkeletonData, SpineError> {
        let c_json = CString::new(json)?;
        let c_skeleton_data =
            unsafe { spSkeletonJson_readSkeletonData(self.c_skeleton_json.0, c_json.as_ptr()) };
        if !c_skeleton_data.is_null() {
            Ok(SkeletonData::new(c_skeleton_data, self.atlas.clone()))
        } else {
            let c_error = unsafe { CStr::from_ptr((*self.c_skeleton_json.0).error) };
            Err(SpineError::new_from_spine(from_c_str(c_error)))
        }
    }

    /// Read the Spine skeleton json data from a file. See [`SkeletonJson::new`] for a full example.
    ///
    /// # Errors
    ///
    /// Returns [`SpineError::ParsingFailed`] if parsing of the json data failed. Returns
    /// [`SpineError::NulError`] if `path` contains an internal 0 byte. Returns
    /// [`SpineError::PathNotUtf8`] if the specified path is not utf-8.
    pub fn read_skeleton_data_file<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<SkeletonData, SpineError> {
        let Some(path_str) = path.as_ref().to_str() else {
            return Err(SpineError::PathNotUtf8);
        };
        let c_path = CString::new(path_str)?;
        let c_skeleton_data =
            unsafe { spSkeletonJson_readSkeletonDataFile(self.c_skeleton_json.0, c_path.as_ptr()) };
        if !c_skeleton_data.is_null() {
            Ok(SkeletonData::new(c_skeleton_data, self.atlas.clone()))
        } else {
            let c_error = unsafe { CStr::from_ptr((*self.c_skeleton_json.0).error) };
            Err(SpineError::new_from_spine(from_c_str(c_error)))
        }
    }

    c_accessor_mut!(
        /// Scales bone positions, image sizes, and translations as they are loaded. This allows
        /// different size images to be used at runtime than were used in Spine.
        ///
        /// See [`Scaling`](http://esotericsoftware.com/spine-loading-skeleton-data#Scaling) in the
        /// Spine Runtimes Guide.
        scale,
        /// Sets the scaling of this skeleton, see [`scale`](`Self::scale`).
        set_scale,
        scale,
        f32
    );
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
