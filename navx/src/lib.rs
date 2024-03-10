#![allow(warnings)]

use std::mem::MaybeUninit;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct NavX {
    handle: MaybeUninit<AHRS>
}

impl NavX {
    pub fn new() -> Self {
        let handle = unsafe {
            let mut instance = std::mem::MaybeUninit::<AHRS>::uninit();
            AHRS_AHRS(instance.as_mut_ptr(), 0);
            instance
        };

        Self {
            handle
        }
    }

    pub fn get_angle(&mut self) -> f64 {
        let angle = unsafe {
            self.handle.assume_init_mut().GetAngle()
        };

        angle
    }
}