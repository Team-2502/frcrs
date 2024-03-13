use std::ffi::CString;

use ctre_sys::{cancoder_wrapper_get_position, cancoder_wrapper_get_absolute_position};


pub struct CanCoder {
    sensor: *mut ctre_sys::ctre_phoenix6_hardware_CANcoder,
}

impl CanCoder {
    pub fn new(id: i32, can_loop: Option<String>) -> Self {
        let sensor = if let Some(can_loop) = can_loop {
            unsafe{ctre_sys::cancoder_wrapper_bind_cancoder_with_bus(id, CString::new(can_loop).unwrap().into_raw())}
        } else {
            unsafe{ctre_sys::cancoder_wrapper_bind_cancoder(id)}
        };

        Self {
            sensor
        }
    }

    pub fn get(&self) -> f64 {
        unsafe {
            cancoder_wrapper_get_position(self.sensor)
        }
    }

    pub fn get_absolute(&self) -> f64 {
        unsafe {
            cancoder_wrapper_get_absolute_position(self.sensor)
        }
    }
}
