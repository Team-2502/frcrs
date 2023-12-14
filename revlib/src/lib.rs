use error::REVError;
use std::ops::RangeInclusive;

use crate::bindings::*;

#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod error;

pub struct SparkMax {
    handle: c_SparkMax_handle,
}

#[derive(Clone, Copy)]
pub enum MotorType {
    Brushed = c_SparkMax_MotorType_c_SparkMax_kBrushed as isize,
    Brushless = c_SparkMax_MotorType_c_SparkMax_kBrushless as isize,
}

#[derive(Clone, Copy)]
pub enum IdleMode {
    Brake = c_SparkMax_IdleMode_c_SparkMax_kBrake as isize,
    Coast = c_SparkMax_IdleMode_c_SparkMax_kCoast as isize,
}

pub enum ControlType {
    DutyCycle = 0,
    Velocity = 1,
    Voltage = 2,
    Position = 3,
    SmartMotion = 4,
    Current = 5,
    SmartVelocity = 6,
}

impl SparkMax {
    pub fn new(can_id: i32, motor_type: MotorType) -> SparkMax {
        unsafe { c_SparkMax_RegisterId(can_id.clone()); };

        SparkMax {
            handle: unsafe { c_SparkMax_Create(can_id, motor_type as u32) },
        }
    }

    pub fn set(&mut self, speed: f32) -> Result<(), REVError> {
        let err = unsafe {
            c_SparkMax_SetpointCommand(
                self.handle,
                speed,
                c_SparkMax_ControlType_c_SparkMax_kDutyCycle,
                0,
                0.0,
                0,
            )
        };

        if err != 0 {
            Err(err.into())
        } else {
            Ok(())
        }
    }

    pub fn stop(&mut self) -> Result<(), REVError> {
        self.set(0.)
    }

    fn set_idle_mode(&mut self, idle_mode: IdleMode) -> Result<(), REVError> {
        let err = unsafe { c_SparkMax_SetIdleMode(self.handle, idle_mode as u32) };

        if err != 0 {
            Err(err.into())
        } else {
            Ok(())
        }
    }

    pub fn set_smart_current_limit(&mut self, limit: u8) -> Result<(), REVError> {
        let err = unsafe { c_SparkMax_SetSmartCurrentLimit(self.handle, limit, 0, 20000) };

        if err != 0 {
            Err(err.into())
        } else {
            Ok(())
        }
    }

    pub fn set_p(&mut self, p: f32) -> Result<(), REVError> {
        unsafe { handle_error!(c_SparkMax_SetP(self.handle, 0, p)) }?;

        Ok(())
    }

    pub fn set_i(&mut self, i: f32) -> Result<(), REVError> {
        unsafe { handle_error!(c_SparkMax_SetP(self.handle, 0, i)) }?;

        Ok(())
    }

    pub fn set_d(&mut self, d: f32) -> Result<(), REVError> {
        unsafe { handle_error!(c_SparkMax_SetP(self.handle, 0, d)) }?;

        Ok(())
    }

    pub fn set_ff(&mut self, ff: f32) -> Result<(), REVError> {
        unsafe { handle_error!(c_SparkMax_SetP(self.handle, 0, ff)) }?;

        Ok(())
    }

    pub fn set_pid_range(&mut self, range: RangeInclusive<f32>) -> Result<(), REVError> {
        unsafe {
            handle_error!(c_SparkMax_SetOutputRange(
                self.handle,
                0,
                *range.start(),
                *range.end()
            ))
        }?;

        Ok(())
    }

    pub fn set_reference(&mut self, value: f32, control_type: ControlType) -> Result<(), REVError> {
        unsafe {
            handle_error!(c_SparkMax_SetpointCommand(
                self.handle,
                value,
                control_type as u32,
                0,
                0.0,
                0
            ))
        }
    }
}

impl Drop for SparkMax {
    fn drop(&mut self) {
        unsafe { c_SparkMax_Destroy(self.handle) }
    }
}
