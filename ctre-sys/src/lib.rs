#![allow(warnings)]

use std::ffi::{c_int, CString};
use std::os::raw::c_void;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

