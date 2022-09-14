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
unsafe fn r_strlen(s: *const c_char) -> size_t {
    libc::strlen(s)
}

#[no_mangle]
unsafe fn r_strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
    libc::strcmp(s1, s2)
}

#[no_mangle]
unsafe fn r_strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int {
    libc::strncmp(s1, s2, n)
}

#[no_mangle]
unsafe fn r_strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int {
    libc::strcasecmp(s1, s2)
}

#[no_mangle]
unsafe fn r_strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    libc::strcpy(dest, src)
}

#[no_mangle]
unsafe fn r_strncat(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char {
    libc::strncat(dest, src, n)
}

#[no_mangle]
unsafe fn r_strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long {
    libc::strtol(nptr, endptr, base)
}

#[no_mangle]
unsafe fn r_strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong {
    libc::strtoul(nptr, endptr, base)
}

#[no_mangle]
unsafe fn r_strrchr(s: *const c_char, c: c_int) -> *mut c_char {
    libc::strrchr(s, c)
}

#[no_mangle]
unsafe fn r_rand() -> c_int {
    libc::rand()
}

#[no_mangle]
fn r_sqrtf(x: c_float) -> c_float {
    x.sqrt()
}

#[no_mangle]
unsafe fn r_malloc(size: size_t) -> *mut c_void {
    libc::malloc(size)
}

#[no_mangle]
unsafe fn r_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    libc::realloc(ptr, size)
}

#[no_mangle]
unsafe fn r_free(ptr: *mut c_void) {
    libc::free(ptr)
}

#[no_mangle]
unsafe fn r_memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    libc::memcpy(dest, src, n)
}

#[no_mangle]
unsafe fn r_memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    libc::memmove(dest, src, n)
}

#[no_mangle]
unsafe fn r_memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    libc::memset(s, c, n)
}

#[no_mangle]
unsafe fn r_printf(_format: *const c_char, _: i32) -> c_int {
    unimplemented!();
}

#[no_mangle]
unsafe fn r_sprintf(_s: *mut c_char, _format: *const c_char, _: i32) -> c_int {
    unimplemented!();
}

#[no_mangle]
unsafe fn r_sscanf(_s: *const c_char, _format: *const c_char, _: i32) -> c_int {
    unimplemented!();
}

#[no_mangle]
unsafe fn r_fopen(filename: *const c_char, modes: *const c_char) -> *mut FILE {
    libc::fopen(filename, modes)
}

#[no_mangle]
unsafe fn r_fclose(stream: *mut FILE) -> c_int {
    libc::fclose(stream)
}

#[no_mangle]
unsafe fn r_fread(ptr: *mut c_void, size: size_t, n: size_t, stream: *mut FILE) -> size_t {
    libc::fread(ptr, size, n, stream)
}

#[no_mangle]
unsafe fn r_fseek(stream: *mut FILE, off: c_long, whence: c_int) -> c_int {
    libc::fseek(stream, off, whence)
}

#[no_mangle]
unsafe fn r_ftell(stream: *mut FILE) -> c_long {
    libc::ftell(stream)
}
