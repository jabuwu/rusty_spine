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

    pub fn c_ptr(&self) -> *mut spSkeletonData {
        self.c_skeleton_data.0
    }
}

impl Drop for SkeletonData {
    fn drop(&mut self) {
        unsafe {
            spSkeletonData_dispose(self.c_skeleton_data.0);
        }
    }
}
