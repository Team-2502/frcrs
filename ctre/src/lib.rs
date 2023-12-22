#![allow(warnings)]

mod error;

use std::ffi::{c_int, c_void, CString};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct Talon {
    handle: *mut c_void
}

impl Talon {
    pub fn new(can_id: i32, can_loop: String) -> Self {
        let model = CString::new("Talon FX").unwrap();
        let can_bus = CString::new(can_loop).unwrap();

        let model_ptr = model.as_ptr();
        let can_bus_ptr = can_bus.as_ptr();

        std::mem::forget(model);
        std::mem::forget(can_bus);

        let handle = unsafe { c_MotController_Create2(
            can_id,
            model_ptr,
            can_bus_ptr
        ) };

        Self {
            handle
        }
    }

    pub fn set(&self, speed: f64) {
        unsafe {
            c_MotController_Set_4(
                self.handle,
                 0,
                 speed,
                 0.0,
                 0 // Demand type neutral
                )
        };
    }

    pub fn stop(&self) {
        self.set(0.0);
    }
}


/*
type ControllerHandle = *mut c_void;

pub struct VictorSPX {
    handle: ControllerHandle,
}

#[derive(Clone, Copy)]
pub enum VictorSPXControlMode {
    PercentOutput = 0,
    Position = 1,
    Velocity = 2,
    Follower = 5,
    MotionProfile = 6,
    MotionMagic = 7,
    MotionProfileArc = 10,
    Disabled = 15,
}

#[derive(Clone, Copy)]
pub enum DemandType {
    Neutral = 0,
    AuxPID = 1,
    ArbitraryFeedForward = 2,
}

#[derive(Clone, Copy)]
pub enum IdleMode {
    EEPROMSetting = 0,
    Coast = 1,
    Brake = 2,
}

impl VictorSPX {
    pub fn new(id: i32) -> Self {
        let model = CString::new("Talon FX").unwrap();
        let can_bus = CString::new("").unwrap();

        let model_ptr = model.as_ptr();
        let can_bus_ptr = can_bus.as_ptr();

        std::mem::forget(model);
        std::mem::forget(can_bus);

        Self {
            handle: unsafe { c_MotController_Create2(id, model_ptr, can_bus_ptr) },
        }
    }

    fn set_percent_ctre(&mut self, speed: f64) -> Result<(), error::Error> {
        self.set(speed, VictorSPXControlMode::PercentOutput)
    }

    pub fn set(&mut self, speed: f64, mode: VictorSPXControlMode) -> Result<(), error::Error> {
        self.set_with_demand(speed, mode, 0.0, DemandType::Neutral)
    }

    pub fn set_with_demand(
        &mut self,
        speed: f64,
        mode: VictorSPXControlMode,
        demand: f64,
        demand_type: DemandType,
    ) -> Result<(), error::Error> {
        let error = unsafe {
            c_MotController_Set_4(
                self.handle,
                mode as i32,
                speed,
                demand,
                demand_type as i32,
            )
        };

        error::to_result(error)
    }

    fn set_idle_mode_ctre(&mut self, idle_mode: IdleMode) {
        unsafe { c_MotController_SetNeutralMode(self.handle, idle_mode as i32) };
    }
}*/