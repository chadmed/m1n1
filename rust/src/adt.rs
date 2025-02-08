// SPDX-License-Identifier: MIT
use core::ffi::*;

use crate::c_size_t;

const ADT_ERR_NOTFOUND: u32 =   1;
const ADT_ERR_BADOFFSET: u32 =  4;
const ADT_ERR_BADPATH: u32 =    5;
const ADT_ERR_BADNCELLS: u32 =  14;
const ADT_ERR_BADVALUE: u32 =   15;
const ADT_ERR_BADLENGTH: u32 =  20;

const ADT_ALIGN: u32 = 4;

pub type Error = ();

#[repr(C)]
#[derive(Debug)]
pub struct ADTProperty {
    name: [c_char; 32],
    size: usize,
    value: [u8],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ADTNodeHeader {
    property_count: u32,
    child_count: u32,
}

#[repr(C, packed(1))]
pub struct ADTSegmentRanges {
    phys: u64,
    iova: u64,
    remap: u64,
    size: u32,
    unk: u32,
}

#[repr(C)]
pub struct ADT {
    _data: (),
}

unsafe extern "C" {
    static adt: *const c_void;
    unsafe fn adt_check_header(adt: *const c_void) -> c_int;
    unsafe fn adt_get_property_count(adt: *const c_void, offset: c_int) -> c_int;
    unsafe fn adt_first_property_offset(adt: *const c_void, offset: c_int) -> c_int;
    unsafe fn adt_next_property_offset(adt: *const c_void, offset: c_int) -> c_int;
    unsafe fn adt_get_property_by_offset(adt: *const c_void, offset: c_int) -> *const c_void;
    unsafe fn adt_get_child_count(adt: *const c_void, offset: c_int) -> c_int;
    unsafe fn adt_first_child_offset(adt: *const c_void, offset: c_int) -> c_int;
    unsafe fn adt_next_sibling_offset(adt: *const c_void, offset: c_int) -> c_int;
    unsafe fn adt_subnode_offset_namelen(adt: *const c_void, parentoffset: c_int, name: *const c_char, namelen: c_size_t) -> c_int;
    unsafe fn adt_path_offset(adt: *const c_void, path: *const c_char) -> c_int;
    unsafe fn adt_path_offset_trace(adt: *const c_void, path: *const c_char, offsets: c_int) -> c_int;
    unsafe fn adt_get_name(adt: *const c_void, nodeoffset: c_int) -> *const c_char;
    unsafe fn adt_get_property_namelen(adt: *const c_void, nodeoffset: c_int, name: *const c_char, namelen: c_size_t) -> *const c_void;
    unsafe fn adt_get_property(adt: *const c_void, nodeoffet: c_int, name: *const c_char) -> *const c_void;
    unsafe fn adt_getprop_by_offset(adt: *const c_void, offset: c_int, namep: *const *const c_char, lenp: *mut c_uint) -> *const c_void;
    unsafe fn adt_getprop_namelen(adt: *const c_void, nodeoffset: c_int, name: *const c_char, namelen: c_size_t) -> *const c_void;
    unsafe fn adt_getprop(adt: *const c_void, nodeoffset: c_int, name: *const c_char, lenp: *mut c_uint) -> *const c_void;
    unsafe fn adt_setprop(adt: *mut c_void, nodeoffset: c_int, name: *const c_char, value: *mut c_void, len: c_size_t) -> c_int;
    unsafe fn adt_getprop_copy(adt: *const c_void, nodeoffset: c_int, name: *const c_char, out: *mut c_void, len: c_size_t) -> c_int;
    unsafe fn adt_get_reg(adt: *const c_void, path: *mut c_int, prop: *const c_char, idx: c_int, addr: u64, size: *mut u64) -> c_int;
    unsafe fn adt_is_compatible(adt: *const c_void, nodeoffset: c_int, compat: *const c_char) -> bool;
}

pub fn check_header() -> Result<(), self::Error> {
    let ret: i32;

    // SAFETY:
    unsafe {
        ret = adt_check_header(adt)
    }

    if ret > 0 {
        return Err(());
    }

    Ok(())
}

pub fn get_prop_count(offset: i32) -> i32 {

}
