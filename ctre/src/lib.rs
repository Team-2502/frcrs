#![allow(warnings)]

mod error;

use std::ffi::{c_int, CString};
use std::os::raw::c_void;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct Talon {
    handle: *mut ctre_phoenix6_hardware_core_CoreTalonFX
}

impl Talon {
    pub fn new(can_id: u8, can_loop: String) -> Self {
        let can_bus = CString::new(can_loop).unwrap();

        let handle = unsafe {
            CreateTalonFX(can_id as c_int, can_bus.as_ptr())
        };

        Self {
            handle
        }
    }

    pub fn set(&self, speed: f64) {
        //let handle = self.handle as *mut c_void;
        unsafe {
            //ctre_phoenix6_hardware_TalonFX_Set(handle, speed);
        };
    }

    pub fn stop(&self) {
        self.set(0.0);
    }
}