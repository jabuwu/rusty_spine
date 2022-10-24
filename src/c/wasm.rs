// Many functions in this file were copied from libc with the following license:

/*-
 * Copyright (c) 1990 The Regents of the University of California.
 * All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Chris Torek.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

use crate::c::FILE;
use std::any::Any;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::sync::{Arc, Mutex, Once};

pub mod types {
    #[allow(non_camel_case_types)]
    pub type c_short = i16;
    #[allow(non_camel_case_types)]
    pub type c_ushort = u16;
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
    #[repr(u8)]
    pub enum c_void {
        __variant1,
        __variant2,
    }
}
use self::types::*;

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

    pub fn malloc(&mut self, size: usize) -> *mut c_void {
        let mut data: Vec<u8> = vec![];
        data.resize(size, 0);
        let ptr = data.as_ptr();
        self.allocations.insert(ptr as *const c_void, data);
        ptr as *mut c_void
    }

    pub unsafe fn realloc(&mut self, ptr: *const c_void, size: usize) -> *mut c_void {
        let mut previous_allocation = self.allocations.remove(&ptr).unwrap();
        previous_allocation.resize(size, 0);
        let new_ptr = previous_allocation.as_ptr();
        self.allocations
            .insert(new_ptr as *const c_void, previous_allocation);
        new_ptr as *mut c_void
    }

    #[allow(dead_code)]
    pub unsafe fn size(&mut self, ptr: *const c_void) -> usize {
        self.allocations.get(&ptr).unwrap().len()
    }

    pub unsafe fn free(&mut self, ptr: *const c_void) {
        self.allocations.remove(&ptr).unwrap();
    }

    #[allow(dead_code)]
    pub fn size_allocated(&self) -> usize {
        let mut size = 0;
        for (_, allocation) in self.allocations.iter() {
            size += allocation.len();
        }
        size
    }
}

#[no_mangle]
pub unsafe extern "C" fn spine_isspace(c: c_int) -> c_int {
    return if c == '\t' as i32
        || c == '\n' as i32
        || c == '\u{b}' as i32
        || c == '\u{c}' as i32
        || c == '\r' as i32
        || c == ' ' as i32
    {
        1 as c_int
    } else {
        0 as c_int
    };
}

#[no_mangle]
pub unsafe extern "C" fn spine_isdigit(c: c_int) -> c_int {
    return ((c as c_uint).wrapping_sub('0' as i32 as c_uint) < 10 as c_int as c_uint) as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn spine_isalpha(c: c_int) -> c_int {
    return if c >= 'a' as i32 && c <= 'z' as i32 || c >= 'A' as i32 && c <= 'Z' as i32 {
        1 as c_int
    } else {
        0 as c_int
    };
}

#[no_mangle]
pub unsafe extern "C" fn spine_isupper(c: c_int) -> c_int {
    return if c >= 'A' as i32 && c <= 'Z' as i32 {
        1 as c_int
    } else {
        0 as c_int
    };
}

#[no_mangle]
pub unsafe extern "C" fn spine_strlen(str: *const c_char) -> c_ulong {
    let mut s: *const c_char;
    s = str;
    while *s != 0 {
        s = s.offset(1);
    }
    return s.offset_from(str) as c_long as c_ulong;
}

#[no_mangle]
pub unsafe extern "C" fn spine_strcmp(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
    loop {
        let fresh0 = s2;
        s2 = s2.offset(1);
        if !(*s1 as c_int == *fresh0 as c_int) {
            break;
        }
        let fresh1 = s1;
        s1 = s1.offset(1);
        if *fresh1 as c_int == 0 as c_int {
            return 0 as c_int;
        }
    }
    s2 = s2.offset(-1);
    return *(s1 as *mut c_uchar) as c_int - *(s2 as *mut c_uchar) as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn spine_strncmp(
    mut s1: *const c_char,
    mut s2: *const c_char,
    mut n: size_t,
) -> c_int {
    if n == 0 as c_int as c_ulong {
        return 0 as c_int;
    }
    loop {
        let fresh0 = s2;
        s2 = s2.offset(1);
        if *s1 as c_int != *fresh0 as c_int {
            s2 = s2.offset(-1);
            return *(s1 as *mut c_uchar) as c_int - *(s2 as *mut c_uchar) as c_int;
        }
        let fresh1 = s1;
        s1 = s1.offset(1);
        if *fresh1 as c_int == 0 as c_int {
            break;
        }
        n = n.wrapping_sub(1);
        if !(n != 0 as c_int as c_ulong) {
            break;
        }
    }
    return 0 as c_int;
}

static mut CHARMAP: [c_uchar; 256] = [
    '\0' as i32 as c_uchar,
    '\u{1}' as i32 as c_uchar,
    '\u{2}' as i32 as c_uchar,
    '\u{3}' as i32 as c_uchar,
    '\u{4}' as i32 as c_uchar,
    '\u{5}' as i32 as c_uchar,
    '\u{6}' as i32 as c_uchar,
    '\u{7}' as i32 as c_uchar,
    '\u{8}' as i32 as c_uchar,
    '\t' as i32 as c_uchar,
    '\n' as i32 as c_uchar,
    '\u{b}' as i32 as c_uchar,
    '\u{c}' as i32 as c_uchar,
    '\r' as i32 as c_uchar,
    '\u{e}' as i32 as c_uchar,
    '\u{f}' as i32 as c_uchar,
    '\u{10}' as i32 as c_uchar,
    '\u{11}' as i32 as c_uchar,
    '\u{12}' as i32 as c_uchar,
    '\u{13}' as i32 as c_uchar,
    '\u{14}' as i32 as c_uchar,
    '\u{15}' as i32 as c_uchar,
    '\u{16}' as i32 as c_uchar,
    '\u{17}' as i32 as c_uchar,
    '\u{18}' as i32 as c_uchar,
    '\u{19}' as i32 as c_uchar,
    '\u{1a}' as i32 as c_uchar,
    '\u{1b}' as i32 as c_uchar,
    '\u{1c}' as i32 as c_uchar,
    '\u{1d}' as i32 as c_uchar,
    '\u{1e}' as i32 as c_uchar,
    '\u{1f}' as i32 as c_uchar,
    ' ' as i32 as c_uchar,
    '!' as i32 as c_uchar,
    '"' as i32 as c_uchar,
    '#' as i32 as c_uchar,
    '$' as i32 as c_uchar,
    '%' as i32 as c_uchar,
    '&' as i32 as c_uchar,
    '\'' as i32 as c_uchar,
    '(' as i32 as c_uchar,
    ')' as i32 as c_uchar,
    '*' as i32 as c_uchar,
    '+' as i32 as c_uchar,
    ',' as i32 as c_uchar,
    '-' as i32 as c_uchar,
    '.' as i32 as c_uchar,
    '/' as i32 as c_uchar,
    '0' as i32 as c_uchar,
    '1' as i32 as c_uchar,
    '2' as i32 as c_uchar,
    '3' as i32 as c_uchar,
    '4' as i32 as c_uchar,
    '5' as i32 as c_uchar,
    '6' as i32 as c_uchar,
    '7' as i32 as c_uchar,
    '8' as i32 as c_uchar,
    '9' as i32 as c_uchar,
    ':' as i32 as c_uchar,
    ';' as i32 as c_uchar,
    '<' as i32 as c_uchar,
    '=' as i32 as c_uchar,
    '>' as i32 as c_uchar,
    '?' as i32 as c_uchar,
    '@' as i32 as c_uchar,
    'a' as i32 as c_uchar,
    'b' as i32 as c_uchar,
    'c' as i32 as c_uchar,
    'd' as i32 as c_uchar,
    'e' as i32 as c_uchar,
    'f' as i32 as c_uchar,
    'g' as i32 as c_uchar,
    'h' as i32 as c_uchar,
    'i' as i32 as c_uchar,
    'j' as i32 as c_uchar,
    'k' as i32 as c_uchar,
    'l' as i32 as c_uchar,
    'm' as i32 as c_uchar,
    'n' as i32 as c_uchar,
    'o' as i32 as c_uchar,
    'p' as i32 as c_uchar,
    'q' as i32 as c_uchar,
    'r' as i32 as c_uchar,
    's' as i32 as c_uchar,
    't' as i32 as c_uchar,
    'u' as i32 as c_uchar,
    'v' as i32 as c_uchar,
    'w' as i32 as c_uchar,
    'x' as i32 as c_uchar,
    'y' as i32 as c_uchar,
    'z' as i32 as c_uchar,
    '[' as i32 as c_uchar,
    '\\' as i32 as c_uchar,
    ']' as i32 as c_uchar,
    '^' as i32 as c_uchar,
    '_' as i32 as c_uchar,
    '`' as i32 as c_uchar,
    'a' as i32 as c_uchar,
    'b' as i32 as c_uchar,
    'c' as i32 as c_uchar,
    'd' as i32 as c_uchar,
    'e' as i32 as c_uchar,
    'f' as i32 as c_uchar,
    'g' as i32 as c_uchar,
    'h' as i32 as c_uchar,
    'i' as i32 as c_uchar,
    'j' as i32 as c_uchar,
    'k' as i32 as c_uchar,
    'l' as i32 as c_uchar,
    'm' as i32 as c_uchar,
    'n' as i32 as c_uchar,
    'o' as i32 as c_uchar,
    'p' as i32 as c_uchar,
    'q' as i32 as c_uchar,
    'r' as i32 as c_uchar,
    's' as i32 as c_uchar,
    't' as i32 as c_uchar,
    'u' as i32 as c_uchar,
    'v' as i32 as c_uchar,
    'w' as i32 as c_uchar,
    'x' as i32 as c_uchar,
    'y' as i32 as c_uchar,
    'z' as i32 as c_uchar,
    '{' as i32 as c_uchar,
    '|' as i32 as c_uchar,
    '}' as i32 as c_uchar,
    '~' as i32 as c_uchar,
    '\u{7f}' as i32 as c_uchar,
    -128i32 as c_uchar,
    -127i32 as c_uchar,
    -126i32 as c_uchar,
    -125i32 as c_uchar,
    -124i32 as c_uchar,
    -123i32 as c_uchar,
    -122i32 as c_uchar,
    -121i32 as c_uchar,
    -120i32 as c_uchar,
    -119i32 as c_uchar,
    -118i32 as c_uchar,
    -117i32 as c_uchar,
    -116i32 as c_uchar,
    -115i32 as c_uchar,
    -114i32 as c_uchar,
    -113i32 as c_uchar,
    -112i32 as c_uchar,
    -111i32 as c_uchar,
    -110i32 as c_uchar,
    -109i32 as c_uchar,
    -108i32 as c_uchar,
    -107i32 as c_uchar,
    -106i32 as c_uchar,
    -105i32 as c_uchar,
    -104i32 as c_uchar,
    -103i32 as c_uchar,
    -102i32 as c_uchar,
    -101i32 as c_uchar,
    -100i32 as c_uchar,
    -99i32 as c_uchar,
    -98i32 as c_uchar,
    -97i32 as c_uchar,
    -96i32 as c_uchar,
    -95i32 as c_uchar,
    -94i32 as c_uchar,
    -93i32 as c_uchar,
    -92i32 as c_uchar,
    -91i32 as c_uchar,
    -90i32 as c_uchar,
    -89i32 as c_uchar,
    -88i32 as c_uchar,
    -87i32 as c_uchar,
    -86i32 as c_uchar,
    -85i32 as c_uchar,
    -84i32 as c_uchar,
    -83i32 as c_uchar,
    -82i32 as c_uchar,
    -81i32 as c_uchar,
    -80i32 as c_uchar,
    -79i32 as c_uchar,
    -78i32 as c_uchar,
    -77i32 as c_uchar,
    -76i32 as c_uchar,
    -75i32 as c_uchar,
    -74i32 as c_uchar,
    -73i32 as c_uchar,
    -72i32 as c_uchar,
    -71i32 as c_uchar,
    -70i32 as c_uchar,
    -69i32 as c_uchar,
    -68i32 as c_uchar,
    -67i32 as c_uchar,
    -66i32 as c_uchar,
    -65i32 as c_uchar,
    -64i32 as c_uchar,
    -63i32 as c_uchar,
    -62i32 as c_uchar,
    -61i32 as c_uchar,
    -60i32 as c_uchar,
    -59i32 as c_uchar,
    -58i32 as c_uchar,
    -57i32 as c_uchar,
    -56i32 as c_uchar,
    -55i32 as c_uchar,
    -54i32 as c_uchar,
    -53i32 as c_uchar,
    -52i32 as c_uchar,
    -51i32 as c_uchar,
    -50i32 as c_uchar,
    -49i32 as c_uchar,
    -48i32 as c_uchar,
    -47i32 as c_uchar,
    -46i32 as c_uchar,
    -45i32 as c_uchar,
    -44i32 as c_uchar,
    -43i32 as c_uchar,
    -42i32 as c_uchar,
    -41i32 as c_uchar,
    -40i32 as c_uchar,
    -39i32 as c_uchar,
    -38i32 as c_uchar,
    -37i32 as c_uchar,
    -36i32 as c_uchar,
    -35i32 as c_uchar,
    -34i32 as c_uchar,
    -33i32 as c_uchar,
    -32i32 as c_uchar,
    -31i32 as c_uchar,
    -30i32 as c_uchar,
    -29i32 as c_uchar,
    -28i32 as c_uchar,
    -27i32 as c_uchar,
    -26i32 as c_uchar,
    -25i32 as c_uchar,
    -24i32 as c_uchar,
    -23i32 as c_uchar,
    -22i32 as c_uchar,
    -21i32 as c_uchar,
    -20i32 as c_uchar,
    -19i32 as c_uchar,
    -18i32 as c_uchar,
    -17i32 as c_uchar,
    -16i32 as c_uchar,
    -15i32 as c_uchar,
    -14i32 as c_uchar,
    -13i32 as c_uchar,
    -12i32 as c_uchar,
    -11i32 as c_uchar,
    -10i32 as c_uchar,
    -9i32 as c_uchar,
    -8i32 as c_uchar,
    -7i32 as c_uchar,
    -6i32 as c_uchar,
    -5i32 as c_uchar,
    -4i32 as c_uchar,
    -3i32 as c_uchar,
    -2i32 as c_uchar,
    -1i32 as c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn spine_strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int {
    let cm: *const c_uchar = CHARMAP.as_ptr();
    let mut us1: *const c_uchar = s1 as *const c_uchar;
    let mut us2: *const c_uchar = s2 as *const c_uchar;
    loop {
        let fresh0 = us2;
        us2 = us2.offset(1);
        if !(*cm.offset(*us1 as isize) as c_int == *cm.offset(*fresh0 as isize) as c_int) {
            break;
        }
        let fresh1 = us1;
        us1 = us1.offset(1);
        if *fresh1 as c_int == '\0' as i32 {
            return 0 as c_int;
        }
    }
    us2 = us2.offset(-1);
    return *cm.offset(*us1 as isize) as c_int - *cm.offset(*us2 as isize) as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn spine_strcpy(mut to: *mut c_char, mut from: *const c_char) -> *mut c_char {
    let save: *mut c_char = to;
    loop {
        *to = *from;
        if !(*to as c_int != '\0' as i32) {
            break;
        }
        from = from.offset(1);
        to = to.offset(1);
    }
    return save;
}

#[no_mangle]
pub unsafe extern "C" fn spine_strncat(
    dst: *mut c_char,
    src: *const c_char,
    mut n: size_t,
) -> *mut c_char {
    if n != 0 as c_int as c_ulong {
        let mut d: *mut c_char = dst;
        let mut s: *const c_char = src;
        while *d as c_int != 0 as c_int {
            d = d.offset(1);
        }
        loop {
            let fresh0 = s;
            s = s.offset(1);
            *d = *fresh0;
            if *d as c_int == 0 as c_int {
                break;
            }
            d = d.offset(1);
            n = n.wrapping_sub(1);
            if !(n != 0 as c_int as c_ulong) {
                break;
            }
        }
        *d = 0 as c_int as c_char;
    }
    return dst;
}

#[no_mangle]
pub unsafe extern "C" fn spine_strtol(
    nptr: *const c_char,
    endptr: *mut *mut c_char,
    mut base: c_int,
) -> c_long {
    let mut s: *const c_char;
    let mut acc: c_long;
    let mut cutoff: c_long;
    let mut c: c_int;
    let neg: c_int;
    let mut any: c_int;
    let mut cutlim: c_int;
    s = nptr;
    loop {
        let fresh0 = s;
        s = s.offset(1);
        c = *fresh0 as c_uchar as c_int;
        if !(spine_isspace(c) != 0) {
            break;
        }
    }
    if c == '-' as i32 {
        neg = 1 as c_int;
        let fresh1 = s;
        s = s.offset(1);
        c = *fresh1 as c_int;
    } else {
        neg = 0 as c_int;
        if c == '+' as i32 {
            let fresh2 = s;
            s = s.offset(1);
            c = *fresh2 as c_int;
        }
    }
    if (base == 0 as c_int || base == 16 as c_int)
        && c == '0' as i32
        && (*s as c_int == 'x' as i32 || *s as c_int == 'X' as i32)
    {
        c = *s.offset(1 as c_int as isize) as c_int;
        s = s.offset(2 as c_int as isize);
        base = 16 as c_int;
    }
    if base == 0 as c_int {
        base = if c == '0' as i32 {
            8 as c_int
        } else {
            10 as c_int
        };
    }
    cutoff = (if neg != 0 {
        (9223372036854775808 as c_ulong).wrapping_neg()
    } else {
        9223372036854775807 as c_long as c_ulong
    }) as c_long;
    cutlim = (cutoff % base as c_long) as c_int;
    cutoff /= base as c_long;
    if neg != 0 {
        if cutlim > 0 as c_int {
            cutlim -= base;
            cutoff += 1 as c_int as c_long;
        }
        cutlim = -cutlim;
    }
    acc = 0 as c_int as c_long;
    any = 0 as c_int;
    loop {
        if spine_isdigit(c) != 0 {
            c -= '0' as i32;
        } else {
            if !(spine_isalpha(c) != 0) {
                break;
            }
            c -= if spine_isupper(c) != 0 {
                'A' as i32 - 10 as c_int
            } else {
                'a' as i32 - 10 as c_int
            };
        }
        if c >= base {
            break;
        }
        if !(any < 0 as c_int) {
            if neg != 0 {
                if acc < cutoff || acc == cutoff && c > cutlim {
                    any = -(1 as c_int);
                    acc = (9223372036854775808 as c_ulong).wrapping_neg() as c_long;
                } else {
                    any = 1 as c_int;
                    acc *= base as c_long;
                    acc -= c as c_long;
                }
            } else if acc > cutoff || acc == cutoff && c > cutlim {
                any = -(1 as c_int);
                acc = 9223372036854775807 as c_long;
            } else {
                any = 1 as c_int;
                acc *= base as c_long;
                acc += c as c_long;
            }
        }
        let fresh3 = s;
        s = s.offset(1);
        c = *fresh3 as c_uchar as c_int;
    }
    if !endptr.is_null() {
        *endptr = (if any != 0 {
            s.offset(-(1 as c_int as isize))
        } else {
            nptr
        }) as *mut c_char;
    }
    return acc;
}

#[no_mangle]
pub unsafe extern "C" fn spine_strtoul(
    nptr: *const c_char,
    endptr: *mut *mut c_char,
    mut base: c_int,
) -> c_ulong {
    let mut s: *const c_char;
    let mut acc: c_ulong;
    let cutoff: c_ulong;
    let mut c: c_int;
    let neg: c_int;
    let mut any: c_int;
    let cutlim: c_int;
    s = nptr;
    loop {
        let fresh0 = s;
        s = s.offset(1);
        c = *fresh0 as c_uchar as c_int;
        if !(spine_isspace(c) != 0) {
            break;
        }
    }
    if c == '-' as i32 {
        neg = 1 as c_int;
        let fresh1 = s;
        s = s.offset(1);
        c = *fresh1 as c_int;
    } else {
        neg = 0 as c_int;
        if c == '+' as i32 {
            let fresh2 = s;
            s = s.offset(1);
            c = *fresh2 as c_int;
        }
    }
    if (base == 0 as c_int || base == 16 as c_int)
        && c == '0' as i32
        && (*s as c_int == 'x' as i32 || *s as c_int == 'X' as i32)
    {
        c = *s.offset(1 as c_int as isize) as c_int;
        s = s.offset(2 as c_int as isize);
        base = 16 as c_int;
    }
    if base == 0 as c_int {
        base = if c == '0' as i32 {
            8 as c_int
        } else {
            10 as c_int
        };
    }
    cutoff = (18446744073709551615 as c_ulong).wrapping_div(base as c_ulong);
    cutlim = (18446744073709551615 as c_ulong).wrapping_rem(base as c_ulong) as c_int;
    acc = 0 as c_int as c_ulong;
    any = 0 as c_int;
    loop {
        if spine_isdigit(c) != 0 {
            c -= '0' as i32;
        } else {
            if !(spine_isalpha(c) != 0) {
                break;
            }
            c -= if spine_isupper(c) != 0 {
                'A' as i32 - 10 as c_int
            } else {
                'a' as i32 - 10 as c_int
            };
        }
        if c >= base {
            break;
        }
        if !(any < 0 as c_int) {
            if acc > cutoff || acc == cutoff && c > cutlim {
                any = -(1 as c_int);
                acc = 18446744073709551615 as c_ulong;
            } else {
                any = 1 as c_int;
                acc = acc.wrapping_mul(base as c_ulong);
                acc = acc.wrapping_add(c as c_ulong);
            }
        }
        let fresh3 = s;
        s = s.offset(1);
        c = *fresh3 as c_uchar as c_int;
    }
    if neg != 0 && any > 0 as c_int {
        acc = acc.wrapping_neg();
    }
    if !endptr.is_null() {
        *endptr = (if any != 0 {
            s.offset(-(1 as c_int as isize))
        } else {
            nptr
        }) as *mut c_char;
    }
    return acc;
}

#[no_mangle]
pub unsafe extern "C" fn spine_strrchr(mut p: *const c_char, ch: c_int) -> *mut c_char {
    let mut save: *mut c_char;
    save = 0 as *mut c_char;
    loop {
        if *p as c_int == ch {
            save = p as *mut c_char;
        }
        if *p == 0 {
            return save;
        }
        p = p.offset(1);
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
    let singleton = Allocator::singleton();
    let mut allocator = singleton.lock().unwrap();
    allocator.malloc(size as usize)
}

#[no_mangle]
unsafe fn spine_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    if !ptr.is_null() {
        let singleton = Allocator::singleton();
        let mut allocator = singleton.lock().unwrap();
        allocator.realloc(ptr, size as usize)
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
unsafe fn spine_free(ptr: *mut c_void) {
    if !ptr.is_null() && ptr != 1 as *mut c_void {
        let singleton = Allocator::singleton();
        let mut allocator = singleton.lock().unwrap();
        allocator.free(ptr)
    }
}

#[no_mangle]
unsafe fn spine_memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    std::ptr::copy_nonoverlapping(src, dest, n as usize);
    dest
}

#[no_mangle]
unsafe fn spine_memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    std::ptr::copy(src, dest, n as usize);
    dest
}

#[no_mangle]
unsafe fn spine_memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    for offset in 0..n {
        (*(s as *mut u8).offset(offset as isize)) = c as u8;
    }
    s
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

fn fmt(format: String, args: Vec<Box<dyn Any>>) -> String {
    let mut new_str = "".to_owned();
    let mut percent = false;
    let mut index = 0;
    for char in format.chars() {
        if char == '%' {
            percent = true;
        } else if percent {
            if let Some(arg) = args.get(index) {
                match char {
                    'i' | 'd' => {
                        if let Some(i) = arg.downcast_ref::<i32>() {
                            new_str += &format!("{}", *i);
                        } else if let Some(i) = arg.downcast_ref::<u32>() {
                            new_str += &format!("{}", *i);
                        } else {
                            panic!("Unsupported printf argument type");
                        }
                    }
                    's' => {
                        if let Some(s) = arg.downcast_ref::<*const c_char>() {
                            new_str +=
                                &format!("{}", unsafe { CStr::from_ptr(*s).to_str().unwrap() });
                        } else {
                            panic!("Unsupported printf argument type");
                        }
                    }
                    'f' => {
                        if let Some(f) = arg.downcast_ref::<f32>() {
                            new_str += &format!("{:.6}", *f);
                        } else if let Some(f) = arg.downcast_ref::<f64>() {
                            new_str += &format!("{:.6}", *f);
                        } else {
                            panic!("Unsupported printf argument type");
                        }
                    }
                    'x' => {
                        if let Some(i) = arg.downcast_ref::<i32>() {
                            new_str += &format!("{:x}", *i);
                        } else if let Some(i) = arg.downcast_ref::<u32>() {
                            new_str += &format!("{:x}", *i);
                        } else {
                            panic!("Unsupported printf argument type");
                        }
                    }
                    _ => {
                        panic!("Unsupported printf tag: %{}", char);
                    }
                }
            } else {
                panic!("Incorrect argument count");
            }
            percent = false;
            index += 1;
        } else {
            new_str.push(char);
        }
    }
    new_str
}

#[cfg_attr(feature = "spine38", allow(dead_code))]
pub(crate) fn printf(c_format: *const c_char, args: Vec<Box<dyn Any>>) {
    let format = unsafe { CStr::from_ptr(c_format).to_str().unwrap().to_owned() };
    print!("{}", fmt(format, args));
}

pub(crate) fn sprintf(c_str: *mut c_char, c_format: *const c_char, args: Vec<Box<dyn Any>>) {
    let format = unsafe { CStr::from_ptr(c_format).to_str().unwrap().to_owned() };
    let result = fmt(format, args);
    unsafe {
        let str = CString::new(result).unwrap();
        spine_strcpy(c_str, str.as_ptr());
    }
}

pub(crate) fn sscanf(c_str: *const c_char, c_format: *const c_char, args: *mut c_uint) {
    let str = unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_owned() };
    let format = unsafe { CStr::from_ptr(c_format).to_str().unwrap().to_owned() };
    assert_eq!(format, "%4x");
    unsafe {
        *args = c_uint::from_str_radix(&str[0..(str.len().min(4))], 16).unwrap();
    }
}

#[cfg_attr(feature = "spine38", allow(unused_macros))]
macro_rules! spine_printf {
    ($format:expr) => {
        crate::c::wasm::printf($format, vec![]);
    };
    ($format:expr, $($arg:expr),+ $(,)? ) => {
        crate::c::wasm::printf($format, vec![
            $(Box::new($arg)),+
        ]);
    };
}

macro_rules! spine_sprintf {
    ($str:expr, $format:expr) => {
        crate::c::wasm::sprintf($str, $format, vec![]);
    };
    ($str:expr, $format:expr, $($arg:expr),+ $(,)? ) => {
        crate::c::wasm::sprintf($str, $format, vec![
            $(Box::new($arg)),+
        ]);
    };
}

macro_rules! spine_sscanf {
    ($str:expr, $format:expr, $a:expr) => {
        crate::c::wasm::sscanf($str, $format, $a);
    };
    ($str:expr, $format:expr, $a:expr,) => {
        crate::c::wasm::sscanf($str, $format, $a);
    };
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use crate::c::{
        c_uint,
        wasm::{spine_strtol, spine_strtoul},
    };

    use super::{spine_strlen, Allocator};

    #[test]
    fn allocator() {
        let mut allocator = Allocator::default();
        let mut allocations = vec![];
        for _ in 0..30 {
            let data = allocator.malloc(255) as *mut u8;
            unsafe {
                for i in 0..255 {
                    *data.offset(i) = i as u8;
                }
                for i in 0..255 {
                    assert_eq!(*data.offset(i), i as u8);
                }
                assert_eq!(allocator.size(data as *const super::c_void), 255);
            }
            allocations.push(data);
        }
        assert_eq!(allocator.size_allocated(), 30 * 255);
        for allocation in allocations.iter() {
            unsafe { allocator.free(*allocation as *const super::c_void) }
        }
        assert_eq!(allocator.size_allocated(), 0);
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

    #[test]
    fn strtol() {
        unsafe {
            let str = CString::new("1234 hello world").unwrap();
            let mut endptr: *mut super::c_char = std::ptr::null_mut();
            let value = spine_strtol(str.as_ptr(), &mut endptr, 10);
            assert_eq!(value, 1234);
            assert_eq!(endptr as *const super::c_char, str.as_ptr().offset(4));

            let str = CString::new("hello world").unwrap();
            let mut endptr: *mut super::c_char = std::ptr::null_mut();
            let value = spine_strtol(str.as_ptr(), &mut endptr, 10);
            assert_eq!(value, 0);
            assert_eq!(endptr as *const super::c_char, str.as_ptr().offset(0));
        }
    }

    #[test]
    fn strtoul() {
        unsafe {
            let str = CString::new("1234 hello world").unwrap();
            let mut endptr: *mut super::c_char = std::ptr::null_mut();
            let value = spine_strtoul(str.as_ptr(), &mut endptr, 10);
            assert_eq!(value, 1234);
            assert_eq!(endptr as *const super::c_char, str.as_ptr().offset(4));

            let str = CString::new("hello world").unwrap();
            let mut endptr: *mut super::c_char = std::ptr::null_mut();
            let value = spine_strtoul(str.as_ptr(), &mut endptr, 10);
            assert_eq!(value, 0);
            assert_eq!(endptr as *const super::c_char, str.as_ptr().offset(0));
        }
    }

    #[test]
    fn fmt() {
        assert_eq!(
            super::fmt("integer: (%i)".to_string(), vec![Box::new(52)]),
            "integer: (52)"
        );
        assert_eq!(
            super::fmt("integer: (%d)".to_string(), vec![Box::new(123)]),
            "integer: (123)"
        );
        assert_eq!(
            super::fmt("float: (%f)".to_string(), vec![Box::new(3.14)]),
            "float: (3.140000)"
        );
        let c_str = CString::new("hello").unwrap();
        assert_eq!(
            super::fmt("string: (%s)".to_string(), vec![Box::new(c_str.as_ptr())]),
            "string: (hello)"
        );
        assert_eq!(
            super::fmt("hex: (%x)".to_string(), vec![Box::new(200)]),
            "hex: (c8)"
        );
    }

    #[test]
    fn sscanf() {
        let c_str = CString::new("3fa5").unwrap();
        let c_format = CString::new("%4x").unwrap();
        let mut uc: c_uint = 0;
        spine_sscanf!(c_str.as_ptr(), c_format.as_ptr(), &mut uc);
        assert_eq!(uc, 16293);

        let c_str = CString::new("3f").unwrap();
        spine_sscanf!(c_str.as_ptr(), c_format.as_ptr(), &mut uc);
        assert_eq!(uc, 63);

        let c_str = CString::new("3fa5ff").unwrap();
        spine_sscanf!(c_str.as_ptr(), c_format.as_ptr(), &mut uc);
        assert_eq!(uc, 16293);
    }
}
