mod joystick;
mod gamepad;

use jni::signature::{Primitive, ReturnType};
pub use joystick::*;
pub use gamepad::*;

use bitvec::prelude::*;

use crate::call::call_static;

#[derive(PartialEq, Clone)]
pub enum RobotMode {
    Disabled,
    Auto,
    Teleop,
    Test,
}

#[derive(PartialEq)]
pub struct RobotState {
    buttons: BitVec,
}

impl RobotState {
    pub fn get() -> Self { 
        let value = call_static!(
            "edu/wpi/first/hal/DriverStationJNI",
            "nativeGetControlWord",
            "()I",
            &[],
            ReturnType::Primitive(Primitive::Int)
        ).i().unwrap();
    
        let mut buttons = bitvec![0; 32];
        buttons[..].store(value);
        Self { buttons } 
    }

    pub fn teleop(&self) -> bool {
        !self.buttons[1]
    }

    pub fn auto(&self) -> bool {
        self.buttons[1]
    }

    pub fn test(&self) -> bool {
        self.buttons[2]
    }

    pub fn enabled(&self) -> bool {
        self.buttons[0]
    }

    pub fn emergency_stop(&self) -> bool {
        self.buttons[3]
    }

    pub fn fms(&self) -> bool {
        self.buttons[4]
    }

    pub fn ds(&self) -> bool {
        self.buttons[5]
    }

    pub fn mode(&self) -> RobotMode {
        if self.enabled() {
            if self.auto() {
                RobotMode::Auto
            } else if self.teleop() {
                RobotMode::Teleop
            } else if self.test() {
                RobotMode::Test
            } else {
                RobotMode::Disabled
            }
        } else {
            RobotMode::Disabled
        }
    }
}

impl std::fmt::Debug for RobotState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let auto = self.auto();
        let teleop = self.teleop();
        let test = self.test();
        let disabled = !self.enabled();
        write!(
            f,
            "RobotState {{ auto: {}, teleop: {}, test: {}, disabled: {} }}",
            auto, teleop, test, disabled
        )
    }
}
