use std::{
    ffi::{CStr, CString},
    path::Path,
    sync::Arc,
};

use crate::{
    atlas::Atlas,
    c::{
        c_uchar, spSkeletonBinary, spSkeletonBinary_create, spSkeletonBinary_dispose,
        spSkeletonBinary_readSkeletonData, spSkeletonBinary_readSkeletonDataFile,
    },
    error::Error,
    skeleton_data::SkeletonData,
    sync_ptr::SyncPtr,
};

/// A loader for Spine binary files.
#[derive(Debug)]
pub struct SkeletonBinary {
    c_skeleton_binary: SyncPtr<spSkeletonBinary>,
    owns_memory: bool,
    atlas: Option<Arc<Atlas>>,
}

impl SkeletonBinary {
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

    c_accessor!(scale, set_scale, scale, f32);
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
