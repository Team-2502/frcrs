pub mod rev;
pub mod robot;

use j4rs_derive::call_from_java;

use std::convert::TryFrom;
use j4rs::prelude::*;
use crate::rev::{IdleMode, MotorType, Spark};

#[call_from_java("frc.robot.Main.rustentry")]
fn entrypoint() { // called on rio boot
    let jvm = Jvm::attach_thread().unwrap();

    // Show "robot code" on driver's station
    jvm.invoke_static("edu.wpi.first.hal.DriverStationJNI", "observeUserProgramStarting", &Vec::new()).unwrap();


    let motor = Spark::new(5, MotorType::Brushless);
    motor.set_idle_mode(IdleMode::Coast);

    loop {
        let teleop: bool = jvm.to_rust(jvm.invoke_static("edu.wpi.first.wpilibj.DriverStation", "isTeleop", &Vec::new()).unwrap()).unwrap();

        match teleop {
            true => {
                motor.set(0.1);
            }
            false => {
                motor.stop();
            }
        };
    }
}
