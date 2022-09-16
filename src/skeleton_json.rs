use std::{
    ffi::{CStr, CString},
    sync::Arc,
};

use crate::{
    atlas::Atlas,
    c::{
        spSkeletonJson, spSkeletonJson_create, spSkeletonJson_dispose,
        spSkeletonJson_readSkeletonData,
    },
    error::Error,
    skeleton_data::SkeletonData,
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct SkeletonJson {
    c_skeleton_json: SyncPtr<spSkeletonJson>,
    atlas: Option<Arc<Atlas>>,
}

impl SkeletonJson {
    pub fn new(atlas: Arc<Atlas>) -> Self {
        let c_skeleton_json = unsafe { spSkeletonJson_create(atlas.c_ptr()) };
        Self {
            c_skeleton_json: SyncPtr(c_skeleton_json),
            atlas: Some(atlas),
        }
    }

    pub fn read_skeleton_data(&self, json: &str) -> Result<SkeletonData, Error> {
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

    c_ptr!(c_skeleton_json, spSkeletonJson);
    c_accessor!(scale, scale_mut, scale, f32);
}

impl Drop for SkeletonJson {
    fn drop(&mut self) {
        unsafe {
            spSkeletonJson_dispose(self.c_skeleton_json.0);
        }
    }
}
