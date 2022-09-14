use std::ffi::{CStr, CString};
use std::{path::Path, ptr::null_mut};

use crate::sync_ptr::SyncPtr;
use crate::{
    c::{c_int, spAtlas, spAtlasPage, spAtlas_create, spAtlas_dispose},
    error::Error,
};

#[derive(Debug)]
pub struct Atlas {
    c_atlas: SyncPtr<spAtlas>,
    pages: Vec<AtlasPage>,
}

impl Atlas {
    pub fn new<P: AsRef<Path>>(data: &[u8], dir: P) -> Result<Atlas, Error> {
        let c_data = CString::new(data)?;
        let c_dir = CString::new(dir.as_ref().to_str().unwrap())?;
        let c_atlas = unsafe {
            spAtlas_create(
                c_data.as_ptr(),
                data.len() as c_int,
                c_dir.as_ptr(),
                null_mut(),
            )
        };
        let mut pages = vec![];
        let mut c_atlas_page = unsafe { (*c_atlas).pages };
        while !c_atlas_page.is_null() {
            pages.push(AtlasPage::new(c_atlas_page));
            c_atlas_page = unsafe { (*c_atlas_page).next };
        }
        Ok(Atlas {
            c_atlas: SyncPtr(c_atlas),
            pages,
        })
    }

    pub fn pages(&self) -> &Vec<AtlasPage> {
        &self.pages
    }

    c_ptr!(c_atlas, spAtlas);
}

impl Drop for Atlas {
    fn drop(&mut self) {
        unsafe {
            spAtlas_dispose(self.c_atlas.0);
        }
    }
}

#[derive(Debug)]
pub struct AtlasPage {
    c_atlas_page: SyncPtr<spAtlasPage>,
}

impl AtlasPage {
    fn new(c_atlas_page: *mut spAtlasPage) -> Self {
        Self {
            c_atlas_page: SyncPtr(c_atlas_page),
        }
    }

    pub fn name(&self) -> &str {
        let c_str = unsafe { CStr::from_ptr((*self.c_atlas_page.0).name) };
        c_str.to_str().unwrap()
    }

    c_ptr!(c_atlas_page, spAtlasPage);
}
