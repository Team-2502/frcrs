







use ctre_sys::{self, talonfx_wrapper_follow, talonfx_wrapper_invert, talonfx_wrapper_get_velocity, talonfx_wrapper_stop, talonfx_wrapper_get_position, talonfx_wrapper_play_tone};
use uom::si::{f64::Angle, angle::revolution};

pub struct Kraken {
    can_id: i32,
    motor: *mut ctre_sys::ctre_phoenix6_hardware_TalonFX,
}

pub enum ControlMode {
    Percent,
    Velocity,
    Position
}

impl Kraken {
    pub fn new(can_id: i32, can_loop: Option<String>) -> Self {

        let motor = unsafe {
            if let Some(bus) = can_loop {
                ctre_sys::talonfx_wrapper_bind_talon_with_bus(can_id, bus.into_bytes().as_mut_ptr())
            } else {
                ctre_sys::talonfx_wrapper_bind_talon(can_id)
                
            }
        };

        Self { can_id, motor }
    }

    pub fn set(&mut self, mode: ControlMode, amount: f64) {
        match mode {
            ControlMode::Percent => {
                unsafe {
                    ctre_sys::talonfx_wrapper_set_speed(self.motor,amount);
                }
            },
            ControlMode::Position => {
                unsafe {
                    ctre_sys::talonfx_wrapper_set_position(self.motor, amount)
                }
            },
            ControlMode::Velocity => {
                unsafe {
                    ctre_sys::talonfx_wrapper_set_velocity(self.motor, amount)
                }
            },
            _ => {unimplemented!()},
            
        }
    }

    pub fn follow(&self, master: Kraken, reverse: bool) {
        unsafe { 
            talonfx_wrapper_follow(self.motor, master.can_id, reverse) 
        }
    }

    pub fn set_inverted(&self, inverted: bool) {
        unsafe {
            talonfx_wrapper_invert(self.motor, inverted);
        }
    }

    /// rotations per second
    pub fn get_speed(&self) -> f64 {
        unsafe {
            talonfx_wrapper_get_velocity(self.motor)
        }
    }

    pub fn stop(&self) {
        unsafe {
            talonfx_wrapper_stop(self.motor);
        }
    }

    /// rotations
    pub fn get_position(&self) -> Angle {
        Angle::new::<revolution>(unsafe {
            talonfx_wrapper_get_position(self.motor)
        })
    }

    /// rotations per second
    pub fn get_velocity(&self) -> f64 {
        unsafe {
            talonfx_wrapper_get_velocity(self.motor)
        }
    }

    pub fn play_tone(&self, hertz: f64) {
        unsafe {
            talonfx_wrapper_play_tone(self.motor, hertz)
        }
    }
}
