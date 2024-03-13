use crate::rev::{ControlType, IdleMode};

use rev_sys::{c_SparkMax_ControlType_c_SparkMax_kPosition, c_SparkMax_SetpointCommand, c_SparkMax_ControlType_c_SparkMax_kDutyCycle, c_SparkMax_ControlType_c_SparkMax_kVelocity, c_SparkMax_GetMotorTemperature, c_SparkMax_GetEncoderVelocity, c_SparkMax_GetEncoderPosition, c_SparkMax_IdleMode_c_SparkMax_kBrake, c_SparkMax_IdleMode_c_SparkMax_kCoast, c_SparkMax_SetIdleMode, c_SparkMax_SetP, c_SparkMax_SetFF, c_SparkMax_SetD, c_SparkMax_SetI, c_SparkMax_handle, c_SparkMax_GetOutputCurrent};
use uom::si::angle;
use uom::si::angle::revolution;
use uom::si::f64::*;

pub struct Spark {
    can_id: i32,
    motor: c_SparkMax_handle,
}

impl Spark {
    pub fn new(can_id: i32) -> Self {
        let brushless = rev_sys::c_SparkMax_MotorType_c_SparkMax_kBrushless;
        let motor = unsafe {
            rev_sys::c_SparkMax_RegisterId(can_id);
            rev_sys::c_SparkMax_Create(can_id, brushless)
        };

        Self { can_id, motor }
    }

    pub fn brushed(can_id: i32) -> Self {
        let brush = rev_sys::c_SparkMax_MotorType_c_SparkMax_kBrushed;
        let motor = unsafe {
            rev_sys::c_SparkMax_RegisterId(can_id);
            rev_sys::c_SparkMax_Create(can_id, brush)
        };

        Self { can_id, motor }
    }

    pub fn get_temperature(&self) -> f32 {
        let mut temp = 0.;
        let ret = unsafe {
            c_SparkMax_GetMotorTemperature(self.motor, &mut temp)
        };

        if ret != 0{
            println!("error on spark {}: {ret}", self.can_id);
        }

        temp
    }

   pub fn set_reference(&self, value: f32, control_type: ControlType) {
        let control_type = match control_type {
            ControlType::Position => c_SparkMax_ControlType_c_SparkMax_kPosition,
            ControlType::Speed => c_SparkMax_ControlType_c_SparkMax_kDutyCycle,
            ControlType::Velocity => c_SparkMax_ControlType_c_SparkMax_kVelocity,
        };

        let ret = unsafe {
            c_SparkMax_SetpointCommand(self.motor, value, control_type, 0, 0., 0)
        };

        if ret != 0{
            println!("error on spark {}: {ret}", self.can_id);
        }
    }

    pub fn get_current(&self) -> f32 {
        let mut current = 0.;
        let ret = unsafe {
            c_SparkMax_GetOutputCurrent(self.motor, &mut current)
        };

        if ret != 0{
            println!("error on spark {}: {ret}", self.can_id);
        }

        current
    }

    pub fn get_velocity(&mut self) -> f32 {
        let mut vel = 0.;

        let ret = unsafe {c_SparkMax_GetEncoderVelocity(self.motor, &mut vel)};

        if ret != 0{
            println!("error on spark {}: {ret}", self.can_id);
        }

        vel
    }

    pub fn get_position(&mut self) -> Angle {
        let mut vel = 0.;

        let ret = unsafe {c_SparkMax_GetEncoderPosition(self.motor, &mut vel)};

        if ret != 0{
            println!("error on spark {}: {ret}", self.can_id);
        }

        Angle::new::<revolution>(vel as f64)
    }

    /// Set the speed of the motor
    ///
    /// `amount` is from -1, 1
    pub fn set(&self, amount: f32) {
        self.set_reference(amount, ControlType::Speed)
    }

    pub fn set_idle_mode(&self, idle_mode: IdleMode) {

        let idle_mode = match idle_mode {
            IdleMode::Brake => c_SparkMax_IdleMode_c_SparkMax_kBrake,
            IdleMode::Coast => c_SparkMax_IdleMode_c_SparkMax_kCoast,
        };

        let ret = unsafe{c_SparkMax_SetIdleMode(self.motor, idle_mode)};

        if ret != 0 {
            println!("error on spark {}: {ret}", self.can_id);
        }
    }

    pub fn set_p(&self, p: f32) {
        let ret = unsafe{c_SparkMax_SetP(self.motor, 0, p)};

        if ret != 0 {
            println!("error on spark {}: {ret}", self.can_id);
        }
    }

    pub fn set_i(&self, i: f32) {
        let ret = unsafe{c_SparkMax_SetI(self.motor, 0, i)};

        if ret != 0 {
            println!("error on spark {}: {ret}", self.can_id);
        }
    }

    pub fn set_d(&self, d: f32) {
        let ret = unsafe{c_SparkMax_SetD(self.motor, 0, d)};

        if ret != 0 {
            println!("error on spark {}: {ret}", self.can_id);
        }
    }

    pub fn set_ff(&self, ff: f32) {
        let ret = unsafe{c_SparkMax_SetFF(self.motor, 0, ff)};

        if ret != 0 {
            println!("error on spark {}: {ret}", self.can_id);
        }
    }

    pub fn follow(&self, _master: Spark, _invert: bool) {
        unimplemented!(); // c_SparkMax_Follow did not get wrapped
        //let ret = unsafe{c_SparkMax_SetFollow(master.motor, followerArbId, followerCfg)};

        //if ret != 0 {
        //    println!("error on spark {}: {ret}", self.can_id);
        //}
    }

    /// Stop the motor
    pub fn stop(&self) {
        unimplemented!(); // c_SparkMax_StopMotor did not get wrapped
    }

    pub fn set_position(&self, position: Angle) {
        self.set_reference(position.get::<angle::revolution>() as f32, ControlType::Position);
    }
}
