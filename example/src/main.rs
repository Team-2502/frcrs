use tracing::warn;
use revlib::MotorType::Brushless;
use revlib::SparkMax;
use frcrs::ds;
use frcrs::ds::{get_robot_state, RobotState};
use frcrs::observe_user_program_starting;
use frcrs::hal_initialize;

fn main() {
    if hal_initialize(500, 0) == 0 {
        panic!("Could not start hal");
    }

    observe_user_program_starting();

    let mut spark = SparkMax::new(2, Brushless);

    loop {
        let state = get_robot_state();

        match state {
            RobotState::Teleop => {
                spark.set(0.05).unwrap()
            }
            RobotState::Auto => {}
            RobotState::Test => {}
            RobotState::Disabled => {
                spark.stop().unwrap();
            }
        }
    }
}
