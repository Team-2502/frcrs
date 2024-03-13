mod joystick;

use hal_sys::{HAL_ControlWord, __BindgenBitfieldUnit};
pub use joystick::*;

use j4rs::{Jvm};
use bitvec::prelude::*;

pub struct RobotState {
    buttons: BitVec,
}

impl RobotState {
    pub fn get() -> Self { 
        let mut word : HAL_ControlWord = HAL_ControlWord { _bitfield_align_1: [], _bitfield_1: __BindgenBitfieldUnit::new([0;4]) };
        let _ret = unsafe {
            hal_sys::HAL_GetControlWord(&mut word)
        };

        let value: i32 = unsafe {std::mem::transmute(word)};

        let mut buttons = bitvec![0; 32];
        buttons[..].store(value);
        Self { buttons } 
    }

    pub fn teleop(&self) -> bool {
        if !self.buttons[1] {
            unsafe { hal_sys::HAL_ObserveUserProgramTeleop() };
        }
        !self.buttons[1]
    }

    pub fn auto(&self) -> bool {
        if self.buttons[1] {
            unsafe { hal_sys::HAL_ObserveUserProgramAutonomous() };
        }
        self.buttons[1]
    }

    pub fn test(&self) -> bool {
        if self.buttons[2] {
            unsafe { hal_sys::HAL_ObserveUserProgramTest() };
        }
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

