use hal_sys::{HAL_ControlWord, HAL_GetControlWord};

use crate::error::{HalError, Result};

fn get_control_word() -> Result<HAL_ControlWord> {
    let mut word = HAL_ControlWord {
        _bitfield_align_1: [0; 0],
        _bitfield_1: HAL_ControlWord::new_bitfield_1(0, 0, 0, 0, 0, 0, 0),
    };

    let status = unsafe { HAL_GetControlWord(&mut word) };

    if status != 0 {
        Err(HalError(status).into())
    } else {
        Ok(word)
    }
}

#[derive(Debug, PartialEq)]
pub enum RobotState {
    Teleop,
    Auto,
    Test,
    Disabled,
}

pub fn get_robot_state() -> RobotState {
    let word = get_control_word().unwrap();

    if !word.enabled() == 1 {
        RobotState::Disabled
    } else if word.autonomous() == 1 {
        RobotState::Auto
    } else if word.test() == 1 {
        RobotState::Test
    } else {
        RobotState::Teleop
    }
}
