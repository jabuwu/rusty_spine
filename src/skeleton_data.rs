use std::sync::Arc;

use crate::{
    atlas::Atlas,
    c::{spSkeletonData, spSkeletonData_dispose},
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct SkeletonData {
    c_skeleton_data: SyncPtr<spSkeletonData>,
    // TODO: this atlas arc is kind of a hack
    // skeleton data should keep a reference to data it requires
    // but that will not be an atlas if a custom attachment loader is used
    _atlas: Option<Arc<Atlas>>,
}

impl SkeletonData {
    pub(crate) fn new(c_skeleton_data: *mut spSkeletonData, atlas: Option<Arc<Atlas>>) -> Self {
        Self {
            c_skeleton_data: SyncPtr(c_skeleton_data),
            _atlas: atlas,
        }
    }

    c_ptr!(c_skeleton_data, spSkeletonData);
    c_accessor_string!(version, version);
    c_accessor_string!(hash, hash);
    c_accessor_string!(images_path, imagesPath);
    c_accessor_string!(audio_path, audioPath);
    c_accessor!(x, x_mut, x, f32);
    c_accessor!(y, y_mut, y, f32);
    c_accessor!(width, width_mut, width, f32);
    c_accessor!(height, height_mut, height, f32);

    // TODO: accessors and methods for the arrays in spSkeletonData
}

impl Drop for SkeletonData {
    fn drop(&mut self) {
        unsafe {
            spSkeletonData_dispose(self.c_skeleton_data.0);
        }
    }
}
