use crate::c::FILE;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::{Arc, Mutex, Once};

pub mod types {
    #[allow(non_camel_case_types)]
    pub type c_short = u32;
    #[allow(non_camel_case_types)]
    pub type c_ushort = i32;
    #[allow(non_camel_case_types)]
    pub type c_int = i32;
    #[allow(non_camel_case_types)]
    pub type c_uint = u32;
    #[allow(non_camel_case_types)]
    pub type c_long = i64;
    #[allow(non_camel_case_types)]
    pub type c_ulong = u64;
    #[allow(non_camel_case_types)]
    pub type c_schar = i8;
    #[allow(non_camel_case_types)]
    pub type c_char = i8;
    #[allow(non_camel_case_types)]
    pub type c_uchar = u8;
    #[allow(non_camel_case_types)]
    pub type c_float = f32;
    #[allow(non_camel_case_types)]
    pub type c_double = f64;
    #[allow(non_camel_case_types)]
    pub type c_void = u8;
}
use types::*;

#[allow(non_camel_case_types)]
type size_t = c_ulong;

#[derive(Default)]
struct Allocator {
    allocations: HashMap<*const c_void, Vec<u8>>,
}

impl Allocator {
    fn singleton() -> Arc<Mutex<Allocator>> {
        static START: Once = Once::new();
        static mut INSTANCE: Option<Arc<Mutex<Allocator>>> = None;
        START.call_once(|| unsafe {
            INSTANCE = Some(Arc::new(Mutex::new(Allocator::default())));
        });
        unsafe {
            let singleton = INSTANCE.as_ref().unwrap();
            singleton.clone()
        }
    }

    pub fn malloc(size: usize) -> *mut c_void {
        let singleton = Self::singleton();
        let mut allocator = singleton.lock().unwrap();
        let mut data: Vec<u8> = vec![];
        data.resize(size, 0);
        let ptr = data.as_ptr();
        allocator.allocations.insert(ptr, data);
        ptr as *mut c_void
    }

    pub unsafe fn realloc(ptr: *const c_void, size: usize) -> *mut c_void {
        let singleton = Self::singleton();
        let mut allocator = singleton.lock().unwrap();
        let mut previous_allocation = allocator.allocations.remove(&ptr).unwrap();
        previous_allocation.resize(size, 0);
        let new_ptr = previous_allocation.as_ptr();
        allocator.allocations.insert(new_ptr, previous_allocation);
        new_ptr as *mut c_void
    }

    #[allow(dead_code)]
    pub unsafe fn size(ptr: *const c_void) -> usize {
        let singleton = Self::singleton();
        let allocator = singleton.lock().unwrap();
        allocator.allocations.get(&ptr).unwrap().len()
    }

    pub unsafe fn free(ptr: *const c_void) {
        let singleton = Self::singleton();
        let mut allocator = singleton.lock().unwrap();
        allocator.allocations.remove(&ptr).unwrap();
    }

    #[allow(dead_code)]
    pub fn size_allocated() -> usize {
        let singleton = Self::singleton();
        let allocator = singleton.lock().unwrap();
        let mut size = 0;
        for (_, allocation) in allocator.allocations.iter() {
            size += allocation.len();
        }
        size
    }
}

#[no_mangle]
unsafe fn spine_strlen(s: *const c_char) -> size_t {
    //libc::strlen(s as *const libc::c_char) as size_t
    let mut len = 0;
    while (*s.offset(len)) != 0 {
        len += 1;
    }
    len as size_t
}

#[no_mangle]
unsafe fn spine_strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
    //libc::strcmp(s1 as *const libc::c_char, s2 as *const libc::c_char) as c_int
    match CStr::from_ptr(s1).cmp(CStr::from_ptr(s2)) {
        Ordering::Equal => 0,
        Ordering::Greater => 1,
        Ordering::Less => -1,
    }
}

#[no_mangle]
unsafe fn spine_strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int {
    /*libc::strncmp(
        s1 as *const libc::c_char,
        s2 as *const libc::c_char,
        n as libc::size_t,
    ) as c_int*/
    let mut str1 = CStr::from_ptr(s1).to_owned().into_string().unwrap();
    let mut str2 = CStr::from_ptr(s2).to_owned().into_string().unwrap();
    str1 = String::from(&str1[0..n as usize]);
    str2 = String::from(&str2[0..n as usize]);
    match str1.cmp(&str2) {
        Ordering::Equal => 0,
        Ordering::Greater => 1,
        Ordering::Less => -1,
    }
}

#[no_mangle]
unsafe fn spine_strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int {
    //libc::strcasecmp(s1 as *const libc::c_char, s2 as *const libc::c_char) as c_int
    match CStr::from_ptr(s1)
        .to_str()
        .unwrap()
        .to_ascii_lowercase()
        .cmp(&CStr::from_ptr(s2).to_str().unwrap().to_ascii_lowercase())
    {
        Ordering::Equal => 0,
        Ordering::Greater => 1,
        Ordering::Less => -1,
    }
}

#[no_mangle]
unsafe fn spine_strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    //libc::strcpy(dest as *mut libc::c_char, src as *const libc::c_char) as *mut c_char
    let mut i = 0;
    while *src.offset(i) != 0 {
        *dest.offset(i) = *src.offset(i);
        i += 1;
    }
    *dest.offset(i) = 0;
    dest
}

#[no_mangle]
unsafe fn spine_strncat(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char {
    /*libc::strncat(
        dest as *mut libc::c_char,
        src as *const libc::c_char,
        n as libc::size_t,
    ) as *mut c_char*/
    let mut o = 0;
    while *dest.offset(o) != 0 {
        o += 1;
    }
    let mut i = 0;
    while *src.offset(i) != 0 && i < n as isize {
        *dest.offset(i + o) = *src.offset(i);
        i += 1;
    }
    while i < n as isize {
        *dest.offset(i + o) = '0' as c_char;
        i += 1;
    }
    *dest.offset(i + o) = 0;
    dest
}

#[no_mangle]
unsafe fn spine_strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long {
    /*libc::strtol(
        nptr as *const libc::c_char,
        endptr as *mut *mut libc::c_char,
        base as libc::c_int,
    ) as libc::c_long*/
    if let Ok(value) = c_long::from_str_radix(CStr::from_ptr(nptr).to_str().unwrap(), base as u32) {
        *endptr = (nptr as *mut c_char).offset(value.to_string().len() as isize);
        value
    } else {
        *endptr = nptr as *mut c_char;
        0
    }
}

#[no_mangle]
unsafe fn spine_strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong {
    /*libc::strtoul(
        nptr as *const libc::c_char,
        endptr as *mut *mut libc::c_char,
        base as libc::c_int,
    ) as c_ulong*/
    if let Ok(value) = c_ulong::from_str_radix(CStr::from_ptr(nptr).to_str().unwrap(), base as u32)
    {
        *endptr = (nptr as *mut c_char).offset(value.to_string().len() as isize);
        value
    } else {
        *endptr = nptr as *mut c_char;
        0
    }
}

#[no_mangle]
unsafe fn spine_strrchr(s: *const c_char, c: c_int) -> *mut c_char {
    //libc::strrchr(s as *const libc::c_char, c as libc::c_int) as *mut c_char
    let mut index = 0;
    loop {
        if *s.offset(index) == c as c_char {
            return s.offset(index) as *mut c_char;
        }
        index += 1;
        if *s.offset(index) == 0 {
            return std::ptr::null_mut();
        }
    }
}

#[no_mangle]
unsafe fn spine_rand() -> c_int {
    unimplemented!();
}

#[no_mangle]
fn spine_sqrtf(x: c_float) -> c_float {
    x.sqrt()
}

#[no_mangle]
unsafe fn spine_malloc(size: size_t) -> *mut c_void {
    //libc::malloc(size as libc::size_t) as *mut c_void
    Allocator::malloc(size as usize)
}

#[no_mangle]
unsafe fn spine_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    //libc::realloc(ptr as *mut libc::c_void, size as libc::size_t) as *mut c_void
    if !ptr.is_null() {
        Allocator::realloc(ptr, size as usize)
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
unsafe fn spine_free(ptr: *mut c_void) {
    //libc::free(ptr as *mut libc::c_void)
    if !ptr.is_null() {
        Allocator::free(ptr)
    }
}

#[no_mangle]
unsafe fn spine_memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    /*libc::memcpy(
        dest as *mut libc::c_void,
        src as *const libc::c_void,
        n as libc::size_t,
    ) as *mut c_void*/
    std::ptr::copy_nonoverlapping(src, dest, n as usize);
    dest
}

#[no_mangle]
unsafe fn spine_memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    /*libc::memmove(
        dest as *mut libc::c_void,
        src as *const libc::c_void,
        n as libc::size_t,
    ) as *mut c_void*/
    std::ptr::copy(src, dest, n as usize);
    dest
}

#[no_mangle]
unsafe fn spine_memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    //libc::memset(s as *mut libc::c_void, c as libc::c_int, n as libc::size_t) as *mut c_void
    for offset in 0..n {
        (*(s as *mut u8).offset(offset as isize)) = c as u8;
    }
    s
}

#[no_mangle]
unsafe fn spine_printf(_format: *const c_char, _: i32) -> c_int {
    unimplemented!();
}

#[no_mangle]
unsafe fn spine_sprintf(_s: *mut c_char, _format: *const c_char, _: i32) -> c_int {
    unimplemented!();
}

#[no_mangle]
unsafe fn spine_sscanf(_s: *const c_char, _format: *const c_char, _: i32) -> c_int {
    unimplemented!();
}

#[no_mangle]
unsafe fn spine_fopen(_filename: *const c_char, _modes: *const c_char) -> *mut FILE {
    unimplemented!();
}

#[no_mangle]
unsafe fn spine_fclose(_stream: *mut FILE) -> c_int {
    unimplemented!();
}

#[no_mangle]
unsafe fn spine_fread(_ptr: *mut c_void, _size: size_t, _n: size_t, _stream: *mut FILE) -> size_t {
    unimplemented!();
}

#[no_mangle]
unsafe fn spine_fseek(_stream: *mut FILE, _off: c_long, _whence: c_int) -> c_int {
    unimplemented!();
}

#[no_mangle]
unsafe fn spine_ftell(_stream: *mut FILE) -> c_long {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::{spine_strlen, Allocator};

    #[test]
    fn allocator() {
        let mut allocations = vec![];
        for _ in 0..30 {
            let data = Allocator::malloc(255);
            unsafe {
                for i in 0..255 {
                    *data.offset(i) = i as u8;
                }
                for i in 0..255 {
                    assert_eq!(*data.offset(i), i as u8);
                }
                assert_eq!(Allocator::size(data), 255);
            }
            allocations.push(data);
        }
        assert_eq!(Allocator::size_allocated(), 30 * 255);
        for allocation in allocations.iter() {
            unsafe { Allocator::free(*allocation) }
        }
        assert_eq!(Allocator::size_allocated(), 0);
    }

    #[test]
    fn strlen() {
        unsafe {
            let empty = CString::new("").unwrap();
            assert_eq!(spine_strlen(empty.as_ptr()), 0);
            let hello_world = CString::new("hello world").unwrap();
            assert_eq!(spine_strlen(hello_world.as_ptr()), 11);
        }
    }
}
