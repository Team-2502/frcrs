use std::process::exit;
use tracing::warn;
use ctre::{Talon};
use revlib::MotorType::Brushless;
use revlib::{MotorType, SparkMax};
use frcrs::ds;
use frcrs::ds::{get_robot_state, RobotState};
use frcrs::observe_user_program_starting;
use frcrs::hal_initialize;
use frcrs::joystick::Joystick;
use navx::NavX;

fn main() {
    if hal_initialize(500, 0) == 0 {
        panic!("Could not start HAL");
    }

    observe_user_program_starting();

    let navx = NavX::new();
    let talon = Talon::new(0, "".to_owned());

    loop {
        let state = get_robot_state();

        match state {
            RobotState::Teleop => {
                println!("{}", navx.get_angle());
            }
            RobotState::Auto => {}
            RobotState::Test => {}
            RobotState::Disabled => {}
        }
    }
}
