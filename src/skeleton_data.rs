use crate::{
    c::{spSkeletonData, spSkeletonData_dispose},
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct SkeletonData {
    c_skeleton_data: SyncPtr<spSkeletonData>,
}

impl SkeletonData {
    pub(crate) fn new(c_skeleton_data: *mut spSkeletonData) -> Self {
        Self {
            c_skeleton_data: SyncPtr(c_skeleton_data),
        }
    }

    c_ptr!(c_skeleton_data, spSkeletonData);
}

impl Drop for SkeletonData {
    fn drop(&mut self) {
        unsafe {
            spSkeletonData_dispose(self.c_skeleton_data.0);
        }
    }
}
