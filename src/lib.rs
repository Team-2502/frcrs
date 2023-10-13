pub mod rev;
pub mod robot;

use j4rs_derive::call_from_java;
use rev::SparkMax;

use std::convert::TryFrom;
use j4rs::prelude::*;
use crate::rev::{IdleMode, MotorType, Spark, SparkPIDController};
use crate::rev::ControlType::Position;

#[call_from_java("frc.robot.Main.rustentry")]
fn entrypoint() { // called on rio boot
    let jvm = Jvm::attach_thread().unwrap();

    // Show "robot code" on driver's station
    jvm.invoke_static("edu.wpi.first.hal.DriverStationJNI", "observeUserProgramStarting", &Vec::new()).unwrap();

    let motor = Spark::new(5, MotorType::Brushless);
    &motor.set_idle_mode(IdleMode::Coast);
    let pid = SparkPIDController::new(&motor);
    pid.set_p(0.004f64);
    pid.set_i(0f64);
    pid.set_d(0f64);

    loop {
        let teleop: bool = jvm.to_rust(jvm.invoke_static("edu.wpi.first.wpilibj.DriverStation", "isTeleop", &Vec::new()).unwrap()).unwrap();

        match teleop {
            true => {
                motor.set(0.1);
                // TODO: get actual pid before testing
                //motor.set_reference(4f64, Position)
            }
            false => {
                motor.stop();
            }
        };
    }
}
