use crate::{c::spAnimation, c_interface::NewFromPtr, sync_ptr::SyncPtr};

#[derive(Debug)]
pub struct Animation {
    c_animation: SyncPtr<spAnimation>,
}

impl NewFromPtr<spAnimation> for Animation {
    unsafe fn new_from_ptr(c_animation: *const spAnimation) -> Self {
        Self {
            c_animation: SyncPtr(c_animation as *mut spAnimation),
        }
    }
}

impl Animation {
    c_ptr!(c_animation, spAnimation);
    c_accessor_string!(name, name);
    // TODO: accessors
}
