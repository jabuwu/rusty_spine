use std::{
    ffi::{CStr, CString},
    path::Path,
    sync::Arc,
};

use crate::{
    c::{
        c_uchar, spSkeletonBinary, spSkeletonBinary_create, spSkeletonBinary_dispose,
        spSkeletonBinary_readSkeletonData, spSkeletonBinary_readSkeletonDataFile,
    },
    c_interface::SyncPtr,
    error::Error,
    skeleton_data::SkeletonData,
    Atlas,
};

/// A loader for Spine binary files.
#[derive(Debug)]
pub struct SkeletonBinary {
    c_skeleton_binary: SyncPtr<spSkeletonBinary>,
    owns_memory: bool,
    atlas: Option<Arc<Atlas>>,
}

impl SkeletonBinary {
    /// Create a new Binary loader using the default atlas attachment loader.
    ///
    /// ```
    /// use std::sync::Arc;
    /// use rusty_spine::{AnimationState, AnimationStateData, Atlas, Error, Skeleton, SkeletonBinary};
    ///
    /// fn load_skeleton() -> Result<(Skeleton, AnimationState), Error> {
    ///     let atlas = Arc::new(Atlas::new_from_file("spineboy.atlas")?);
    ///     let skeleton_binary = SkeletonBinary::new(atlas);
    ///     let skeleton_data = Arc::new(skeleton_binary.read_skeleton_data_file("spineboy.skel")?);
    ///     let animation_state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));
    ///     let skeleton = Skeleton::new(skeleton_data);
    ///     let animation_state = AnimationState::new(animation_state_data);
    ///     Ok((skeleton, animation_state))
    /// }
    /// ```
    pub fn new(atlas: Arc<Atlas>) -> Self {
        let c_skeleton_binary = unsafe { spSkeletonBinary_create(atlas.c_ptr()) };
        Self {
            c_skeleton_binary: SyncPtr(c_skeleton_binary),
            owns_memory: true,
            atlas: Some(atlas),
        }
    }

    pub fn read_skeleton_data(&self, data: &[u8]) -> Result<SkeletonData, Error> {
        let c_skeleton_data = unsafe {
            spSkeletonBinary_readSkeletonData(
                self.c_skeleton_binary.0,
                data.as_ptr() as *const c_uchar,
                data.len() as i32,
            )
        };
        if !c_skeleton_data.is_null() {
            Ok(SkeletonData::new(c_skeleton_data, self.atlas.clone()))
        } else {
            let c_error = unsafe { CStr::from_ptr((*self.c_skeleton_binary.0).error) };
            Err(Error::new_from_spine(c_error.to_str().unwrap()))
        }
    }

    pub fn read_skeleton_data_file<P: AsRef<Path>>(&self, path: P) -> Result<SkeletonData, Error> {
        let c_path = CString::new(path.as_ref().to_str().unwrap())?;
        let c_skeleton_data = unsafe {
            spSkeletonBinary_readSkeletonDataFile(self.c_skeleton_binary.0, c_path.as_ptr())
        };
        if !c_skeleton_data.is_null() {
            Ok(SkeletonData::new(c_skeleton_data, self.atlas.clone()))
        } else {
            let c_error = unsafe { CStr::from_ptr((*self.c_skeleton_binary.0).error) };
            Err(Error::new_from_spine(c_error.to_str().unwrap()))
        }
    }

    c_accessor_mut!(scale, set_scale, scale, f32);
    c_ptr!(c_skeleton_binary, spSkeletonBinary);
}

impl Drop for SkeletonBinary {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                spSkeletonBinary_dispose(self.c_skeleton_binary.0);
            }
        }
    }
}
