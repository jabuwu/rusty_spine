pub mod types {
    #[allow(non_camel_case_types)]
    pub type c_short = libc::c_short;
    #[allow(non_camel_case_types)]
    pub type c_ushort = libc::c_ushort;
    #[allow(non_camel_case_types)]
    pub type c_int = libc::c_int;
    #[allow(non_camel_case_types)]
    pub type c_uint = libc::c_uint;
    #[allow(non_camel_case_types)]
    pub type c_long = libc::c_long;
    #[allow(non_camel_case_types)]
    pub type c_ulong = libc::c_ulong;
    #[allow(non_camel_case_types)]
    pub type c_schar = libc::c_schar;
    #[allow(non_camel_case_types)]
    pub type c_char = libc::c_char;
    #[allow(non_camel_case_types)]
    pub type c_uchar = libc::c_uchar;
    #[allow(non_camel_case_types)]
    pub type c_float = libc::c_float;
    #[allow(non_camel_case_types)]
    pub type c_double = libc::c_double;
    #[allow(non_camel_case_types)]
    pub type c_void = libc::c_void;
}
use types::*;

#[allow(non_camel_case_types)]
type size_t = libc::size_t;
#[allow(non_camel_case_types)]
type FILE = libc::FILE;

#[no_mangle]
unsafe fn spine_strlen(s: *const c_char) -> size_t {
    libc::strlen(s)
}

#[no_mangle]
unsafe fn spine_strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
    libc::strcmp(s1, s2)
}

#[no_mangle]
unsafe fn spine_strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int {
    libc::strncmp(s1, s2, n)
}

#[no_mangle]
unsafe fn spine_strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int {
    libc::strcasecmp(s1, s2)
}

#[no_mangle]
unsafe fn spine_strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    libc::strcpy(dest, src)
}

#[no_mangle]
unsafe fn spine_strncat(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char {
    libc::strncat(dest, src, n)
}

#[no_mangle]
unsafe fn spine_strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long {
    libc::strtol(nptr, endptr, base)
}

#[no_mangle]
unsafe fn spine_strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong {
    libc::strtoul(nptr, endptr, base)
}

#[no_mangle]
unsafe fn spine_strrchr(s: *const c_char, c: c_int) -> *mut c_char {
    libc::strrchr(s, c)
}

#[no_mangle]
unsafe fn spine_rand() -> c_int {
    libc::rand()
}

#[no_mangle]
fn spine_sqrtf(x: c_float) -> c_float {
    x.sqrt()
}

#[no_mangle]
unsafe fn spine_malloc(size: size_t) -> *mut c_void {
    libc::malloc(size)
}

#[no_mangle]
unsafe fn spine_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    libc::realloc(ptr, size)
}

#[no_mangle]
unsafe fn spine_free(ptr: *mut c_void) {
    libc::free(ptr)
}

#[no_mangle]
unsafe fn spine_memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    libc::memcpy(dest, src, n)
}

#[no_mangle]
unsafe fn spine_memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    libc::memmove(dest, src, n)
}

#[no_mangle]
unsafe fn spine_memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    libc::memset(s, c, n)
}

#[cfg_attr(feature="spine38", allow(unused_macros))]
macro_rules! spine_printf {
    ($format:expr) => {
        libc::printf($format);
    };
    ($format:expr, $($arg:expr),+ $(,)? ) => {
        libc::printf($format, $($arg),+);
    };
}

macro_rules! spine_sprintf {
    ($str:expr, $format:expr) => {
        libc::sprintf($str, $format);
    };
    ($str:expr, $format:expr, $($arg:expr),+ $(,)? ) => {
        libc::sprintf($str, $format, $($arg),+);
    };
}

macro_rules! spine_sscanf {
    ($str:expr, $format:expr) => {
        libc::sscanf($str, $format);
    };
    ($str:expr, $format:expr, $($arg:expr),+ $(,)? ) => {
        libc::sscanf($str, $format, $($arg),+);
    };
}

#[no_mangle]
unsafe fn spine_fopen(filename: *const c_char, modes: *const c_char) -> *mut FILE {
    libc::fopen(filename, modes)
}

#[no_mangle]
unsafe fn spine_fclose(stream: *mut FILE) -> c_int {
    libc::fclose(stream)
}

#[no_mangle]
unsafe fn spine_fread(ptr: *mut c_void, size: size_t, n: size_t, stream: *mut FILE) -> size_t {
    libc::fread(ptr, size, n, stream)
}

#[no_mangle]
unsafe fn spine_fseek(stream: *mut FILE, off: c_long, whence: c_int) -> c_int {
    libc::fseek(stream, off, whence)
}

#[no_mangle]
unsafe fn spine_ftell(stream: *mut FILE) -> c_long {
    libc::ftell(stream)
}
