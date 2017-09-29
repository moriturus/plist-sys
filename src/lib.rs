/*
The MIT License (MIT)

Copyright (c) 2017 Henrique Sasaki Yuya

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

//! Raw bindings to the [libplist](http://www.libimobiledevice.org) library.

#![feature(libc)]

extern crate libc;

pub mod plist_type;

use libc::{c_char, c_double, c_void, int32_t, int8_t, uint32_t, uint64_t, uint8_t};
pub use plist_type::PlistType;

pub type Plist = *mut c_void;
pub type PlistDictIter = *mut c_void;

// Creation and destruction.
extern "C" {
    pub fn plist_new_dict() -> Plist;
    pub fn plist_new_array() -> Plist;
    pub fn plist_new_string(val: *const c_char) -> Plist;
    pub fn plist_new_bool(val: uint8_t) -> Plist;
    pub fn plist_new_uint(val: uint64_t) -> Plist;
    pub fn plist_new_real(val: c_double) -> Plist;
    pub fn plist_new_data(val: *const c_char, length: uint64_t) -> Plist;
    pub fn plist_new_date(sec: int32_t, usec: int32_t) -> Plist;
    pub fn plist_new_uid(val: uint64_t) -> Plist;
    pub fn plist_free(plist: Plist);
    pub fn plist_copy(node: Plist) -> Plist;
}

// Array functions
extern "C" {
    pub fn plist_array_get_size(node: Plist) -> uint32_t;
    pub fn plist_array_get_item(node: Plist, n: uint32_t) -> Plist;
    pub fn plist_array_get_item_index(node: Plist) -> uint32_t;
    pub fn plist_array_set_item(node: Plist, item: Plist, n: uint32_t);
    pub fn plist_array_append_item(node: Plist, item: Plist);
    pub fn plist_array_insert_item(node: Plist, item: Plist, n: uint32_t);
    pub fn plist_array_remove_item(node: Plist, n: uint32_t);
}

// Dictionary functions
extern "C" {
    pub fn plist_dict_get_size(node: Plist) -> uint32_t;
    pub fn plist_dict_new_iter(node: Plist, iter: *mut PlistDictIter);
    pub fn plist_dict_next_item(
        node: Plist,
        iter: PlistDictIter,
        key: *mut *mut c_char,
        val: *mut Plist,
    );
    pub fn plist_dict_get_item_key(node: Plist, key: *mut *mut c_char);
    pub fn plist_dict_get_item(node: Plist, key: *const c_char) -> Plist;
    pub fn plist_dict_set_item(node: Plist, key: *const c_char, item: Plist);
    pub fn plist_dict_remove_item(node: Plist, key: *const c_char);
    pub fn plist_dict_merge(target: *mut Plist, source: Plist);
}

// Getters
extern "C" {
    pub fn plist_get_parent(node: Plist) -> Plist;
    pub fn plist_get_node_type(node: Plist) -> PlistType;
    pub fn plist_get_key_val(node: Plist, val: *mut *mut c_char);
    pub fn plist_get_string_val(node: Plist, val: *mut *mut c_char);
    pub fn plist_get_bool_val(node: Plist, val: *mut uint8_t);
    pub fn plist_get_uint_val(node: Plist, val: *mut uint64_t);
    pub fn plist_get_real_val(node: Plist, val: *mut c_double);
    pub fn plist_get_data_val(node: Plist, val: *mut *mut c_char, length: *mut uint64_t);
    pub fn plist_get_date_val(node: Plist, sec: *mut int32_t, usec: *mut int32_t);
    pub fn plist_get_uid_val(node: Plist, val: *mut uint64_t);
}

// Setters
extern "C" {
    pub fn plist_set_key_val(node: Plist, val: *const c_char);
    pub fn plist_set_string_val(node: Plist, val: *const c_char);
    pub fn plist_set_bool_val(node: Plist, val: uint8_t);
    pub fn plist_set_uint_val(node: Plist, val: uint64_t);
    pub fn plist_set_real_val(node: Plist, val: c_double);
    pub fn plist_set_data_val(node: Plist, val: *const c_char, length: uint64_t);
    pub fn plist_set_date_val(node: Plist, sec: int32_t, usec: int32_t);
    pub fn plist_set_uid_val(node: Plist, val: uint64_t);
}

// Import and export
extern "C" {
    pub fn plist_to_xml(plist: Plist, plist_xml: *mut *mut c_char, length: *mut uint32_t);
    pub fn plist_to_bin(plist: Plist, plist_bin: *mut *mut c_char, length: *mut uint32_t);
    pub fn plist_from_xml(plist_xml: *const c_char, length: uint32_t, plist: *mut Plist);
    pub fn plist_from_bin(plist_bin: *const c_char, length: uint32_t, plist: *mut Plist);
    pub fn plist_from_memory(plist_data: *const c_char, length: uint32_t, plist: *mut Plist);
    pub fn plist_is_binary(plist_data: *const c_char, length: uint32_t) -> int8_t;
}

// Utils
extern "C" {
    pub fn plist_access_path(plist: Plist, length: uint32_t, ...) -> Plist;
    // pub fn plist_access_pathv(plist: Plist length: uint32_t, v: va_list) -> Plist;
    pub fn plist_compare_node_value(node_l: Plist, node_r: Plist) -> c_char;
}

/// Compares `node`'s type with `plist_type`.
///
/// If `node` is null, returns `false`.
pub unsafe fn plist_is_type(node: Plist, plist_type: PlistType) -> bool {
    !node.is_null() && plist_get_node_type(node) == plist_type
}

#[test]
fn test_plist_is_type_boolean() {
    unsafe {
        let n = plist_new_bool(0);
        assert!(plist_is_type(n, PlistType::Boolean));
    }
}

#[test]
#[should_panic]
fn test_plist_is_type_boolean_panic() {
    unsafe {
        let n = plist_new_uint(0);
        assert!(plist_is_type(n, PlistType::Boolean));
    }
}

#[test]
fn test_plist_is_type_uint() {
    unsafe {
        let n = plist_new_uint(0);
        assert!(plist_is_type(n, PlistType::UInt));
    }
}

#[test]
#[should_panic]
fn test_plist_is_type_uint_panic() {
    unsafe {
        let n = plist_new_real(0.0);
        assert!(plist_is_type(n, PlistType::UInt));
    }
}

#[test]
fn test_plist_is_type_real() {
    unsafe {
        let n = plist_new_real(0.0);
        assert!(plist_is_type(n, PlistType::Real));
    }
}

#[test]
#[should_panic]
fn test_plist_is_type_real_panic() {
    unsafe {
        use std::ffi::CString;
        let p = CString::new("hello").expect("c style string");
        let n = plist_new_string(p.as_ptr());
        assert!(plist_is_type(n, PlistType::Real));
    }
}

#[test]
fn test_plist_is_type_string() {
    unsafe {
        use std::ffi::CString;
        let p = CString::new("hello").expect("c style string");
        let n = plist_new_string(p.as_ptr());
        assert!(plist_is_type(n, PlistType::String));
    }
}

#[test]
#[should_panic]
fn test_plist_is_type_string_panic() {
    unsafe {
        use std::ffi::CString;
        let p = CString::new("hello").expect("c style string");
        let n = plist_new_string(p.as_ptr());
        assert!(plist_is_type(n, PlistType::Array));
    }
}

#[test]
fn test_plist_is_type_array() {
    unsafe {
        let n = plist_new_array();
        assert!(plist_is_type(n, PlistType::Array));
    }
}

#[test]
#[should_panic]
fn test_plist_is_type_array_panic() {
    unsafe {
        let n = plist_new_array();
        assert!(plist_is_type(n, PlistType::Dictionary));
    }
}

#[test]
fn test_plist_is_type_dictionary() {
    unsafe {
        let n = plist_new_dict();
        assert!(plist_is_type(n, PlistType::Dictionary));
    }
}

#[test]
#[should_panic]
fn test_plist_is_type_dictionary_panic() {
    unsafe {
        let n = plist_new_dict();
        assert!(plist_is_type(n, PlistType::Date));
    }
}

#[test]
fn test_plist_is_type_date() {
    unsafe {
        let n = plist_new_date(0, 0);
        assert!(plist_is_type(n, PlistType::Date));
    }
}

#[test]
fn test_plist_is_type_data() {
    unsafe {
        use std::ffi::CString;
        let r = b"hello".to_vec();
        let d = CString::from_vec_unchecked(r);
        let n = plist_new_data(d.as_ptr(), d.as_bytes().len() as _);
        assert!(plist_is_type(n, PlistType::Data));
    }
}

#[test]
#[should_panic]
fn test_plist_is_type_data_panic() {
    unsafe {
        use std::ffi::CString;
        let r = b"hello".to_vec();
        let d = CString::from_vec_unchecked(r);
        let n = plist_new_data(d.as_ptr(), d.as_bytes().len() as _);
        assert!(plist_is_type(n, PlistType::Key));
    }
}
