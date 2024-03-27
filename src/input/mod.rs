mod joystick;

use jni::signature::{Primitive, ReturnType};
pub use joystick::*;
use crate::call::call_static;

use j4rs::{Instance, InvocationArg, Jvm};
use bitvec::prelude::*;

pub struct RobotState {
    buttons: BitVec,
}

impl RobotState {
    pub fn get() -> Self { 
        let value = call_static!(
            "edu/wpi/first/hal/DriverStationJNI",
            "nativeGetControlWord",
            "(I)V",
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
}

