// SPDX-License-Identifier: MIT
use core::ffi::*;
use core::mem::size_of;

use crate::c_size_t;

const ADT_ERR_NOTFOUND: i32 = 1;
const ADT_ERR_BADOFFSET: i32 = 4;
const ADT_ERR_BADPATH: i32 = 5;
const ADT_ERR_BADNCELLS: i32 = 14;
const ADT_ERR_BADVALUE: i32 = 15;
const ADT_ERR_BADLENGTH: i32 = 20;

const ADT_ALIGN: i32 = 4;

/// Assume a maximum ADT size of 512 K
const ADT_MAX_SIZE: usize = 512 * 1024;

pub type Error = ();

#[repr(C)]
#[derive(Debug)]
pub struct ADTProperty {
    name: [c_char; 32],
    size: c_size_t,
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
pub struct ADT<'a> {
    data: &'a [u8],
}

impl<'a> ADT<'a> {
    /// Not required... yet
    pub fn from_raw(ptr: *const ADT) -> Self {
        // SAFETY: ptr comes from iBoot, we know it points to the ADT
        unsafe {
            Self {
                data: core::slice::from_raw_parts(ptr as *const u8, ADT_MAX_SIZE),
            }
        }
    }

    /// Reimplementations of private functions
    fn check_node(node: *const ADTNodeHeader) -> i32 {
        if node as usize + size_of::<ADTNodeHeader>() > ADT_MAX_SIZE {
            return -ADT_ERR_BADOFFSET;
        }

        // SAFETY: by the time we've called this, we already know we have
        // an aligned, valid node
        unsafe {
            if (*node).property_count > 2048
                || (*node).property_count == 0
                || (*node).child_count > 2048
            {
                return -ADT_ERR_BADOFFSET;
            }
        }
        0
    }

    /// The ADT header is just a node at the base of the ADT
    fn check_header() -> i32 {
        // SAFETY: adt is always valid
        unsafe {
            ADT::check_node(adt as *const ADTNodeHeader)
        }
    }

    fn check_offset(offset: i32) -> i32 {
        if offset < 0 || (offset % ADT_ALIGN != 0) {
            -ADT_ERR_BADOFFSET
        } else {
            0
        }
    }

    fn check_property(prop: *const ADTProperty) -> i32 {
        if prop as *const u8 as u64 == 0 {
            return -ADT_ERR_BADOFFSET;
        }

        // SAFETY: We know the pointer is not null
        unsafe {
            if prop as *const u8 as usize + 32 + 64 + (*prop).size > ADT_MAX_SIZE
                || ((*prop).size & 0x7ff00000) != 0
            {
                return -ADT_ERR_BADOFFSET;
            } else {
                0
            }
        }
    }

    /// Reimplementation of the ADT_NODE(adt, offset) macro in adt.h.
    pub fn node_at(offset: i32) -> Result<&'static ADTNodeHeader, i32> {
        let mut ret: i32;
        ret = ADT::check_offset(offset);
        if ret < 0 {
            return Err(ret);
        }

        // SAFETY: adt is always a valid pointer
        unsafe {
            let node = adt.add(offset as usize) as *const ADTNodeHeader;
            ret = ADT::check_node(node);
            match ret {
                0 => Ok(&*node),
                _ => Err(ret),
            }
        }
    }

    /// Wrapper around wrapper around ADT_PROP(adt, offset) macro.
    /// Returns a reference to an ADTProperty on success, and an ADT errno
    /// on failure
    pub fn property_at(offset: i32) -> Result<&'static ADTProperty, i32> {
        let mut ret: i32;
        ret = ADT::check_offset(offset);
        if ret < 0 {
            return Err(ret);
        }

        // SAFETY: we can assume the call to adt_get_property_by_offset is safe
        // as we know adt is a valid pointer, and we've checked that the offset
        // is not null or misaligned
        unsafe {
            let prop = adt_get_property_by_offset(adt, offset);
            ret = ADT::check_property(prop);
            match ret {
                0 => Ok(&*prop),
                _ => Err(ret),
            }
        }
    }

    pub fn value_at(offset: i32) -> Option<&'static [u8]> {
        let prop = ADT::property_at(offset);
        // SAFETY: If we get a property, then the property is safe. We know the
        // size of p.value in bytes.
        unsafe {
            match prop {
                Ok(p) => Some(core::slice::from_raw_parts(p.value.as_ptr(), p.size)),
                Err(_) => None,
            }
        }
    }

    pub fn get_property_by_name(node: i32, name: &str) -> Result<&'static ADTProperty, i32> {
        unsafe {
            let prop = adt_get_property(adt, node, name.as_ptr());
            let ret = ADT::check_property(prop);
            match ret {
                0 => Ok(&*prop),
                _ => Err(ret),
            }
        }
    }
}

extern "C" {
    static adt: *const ADT<'static>; // Global, immutable
}

unsafe extern "C" {
    unsafe fn adt_get_property_count(adt: *const ADT, offset: c_int) -> c_int;
    unsafe fn adt_first_property_offset(adt: *const ADT, offset: c_int) -> c_int;
    unsafe fn adt_next_property_offset(adt: *const ADT, offset: c_int) -> c_int;
    unsafe fn adt_get_property_by_offset(adt: *const ADT, offset: c_int) -> *const ADTProperty;
    unsafe fn adt_get_child_count(adt: *const ADT, offset: c_int) -> c_int;
    unsafe fn adt_first_child_offset(adt: *const ADT, offset: c_int) -> c_int;
    unsafe fn adt_next_sibling_offset(adt: *const ADT, offset: c_int) -> c_int;
    unsafe fn adt_subnode_offset_namelen(
        adt: *const ADT,
        parentoffset: c_int,
        name: *const c_char,
        namelen: c_size_t,
    ) -> c_int;
    unsafe fn adt_path_offset(adt: *const ADT, path: *const c_char) -> c_int;
    unsafe fn adt_path_offset_trace(adt: *const ADT, path: *const c_char, offsets: c_int) -> c_int;
    unsafe fn adt_get_name(adt: *const ADT, nodeoffset: c_int) -> *const c_char;
    unsafe fn adt_get_property_namelen(
        adt: *const ADT,
        nodeoffset: c_int,
        name: *const c_char,
        namelen: c_size_t,
    ) -> *const ADTProperty;
    unsafe fn adt_get_property(
        adt: *const ADT,
        nodeoffet: c_int,
        name: *const c_char,
    ) -> *const ADTProperty;
    unsafe fn adt_getprop_by_offset(
        adt: *const ADT,
        offset: c_int,
        namep: *const *const c_char,
        lenp: *mut c_uint,
    ) -> *const ADTProperty;
    unsafe fn adt_getprop_namelen(
        adt: *const ADT,
        nodeoffset: c_int,
        name: *const c_char,
        namelen: c_size_t,
    ) -> *const c_void;
    unsafe fn adt_getprop(
        adt: *const ADT,
        nodeoffset: c_int,
        name: *const c_char,
        lenp: *mut c_uint,
    ) -> *const ADTProperty;
    unsafe fn adt_setprop(
        adt: *mut ADT,
        nodeoffset: c_int,
        name: *const c_char,
        value: *mut c_void,
        len: c_size_t,
    ) -> c_int;
    unsafe fn adt_getprop_copy(
        adt: *const ADT,
        nodeoffset: c_int,
        name: *const c_char,
        out: *mut c_void,
        len: c_size_t,
    ) -> c_int;
    unsafe fn adt_get_reg(
        adt: *const ADT,
        path: *mut c_int,
        prop: *const c_char,
        idx: c_int,
        addr: u64,
        size: *mut u64,
    ) -> c_int;
    unsafe fn adt_is_compatible(adt: *const ADT, nodeoffset: c_int, compat: *const c_char) -> bool;
}
