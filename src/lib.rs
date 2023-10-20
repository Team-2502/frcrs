pub mod ctre;
pub mod input;
pub mod networktables;
pub mod rev;
pub mod robot;

use input::Joystick;
use j4rs_derive::call_from_java;
use networktables::SmartDashboard;
use rev::SparkMax;

use crate::rev::ControlType::Position;
use crate::rev::{IdleMode, MotorType, Spark, SparkPIDController};
use j4rs::prelude::*;
use std::convert::TryFrom;
use uom::si::angle::degree;
use uom::si::angle::revolution;
use uom::si::f64::*;

#[call_from_java("frc.robot.Main.rustentry")]
fn entrypoint() {
    // called on rio boot
    let jvm = Jvm::attach_thread().unwrap();

    // Show "robot code" on driver's station
    jvm.invoke_static(
        "edu.wpi.first.hal.DriverStationJNI",
        "observeUserProgramStarting",
        &Vec::new(),
    )
    .unwrap();

    let spark = Spark::new(5, MotorType::Brushless);
    spark.set_idle_mode(IdleMode::Coast);
    let pid = SparkPIDController::new(&spark);
    pid.set_p(0.04f64);
    pid.set_i(0f64);
    pid.set_d(0f64);
    let joystick = Joystick::new(0);

    loop {
        let teleop: bool = jvm
            .to_rust(
                jvm.invoke_static(
                    "edu.wpi.first.wpilibj.DriverStation",
                    "isTeleop",
                    &Vec::new(),
                )
                .unwrap(),
            )
            .unwrap();

        match teleop {
            true => {
                if joystick.get(1) {
                    println!("trigger");
                    spark.set_position(Angle::new::<degree>(180.0));
                } else {
                    spark.set_position(Angle::new::<revolution>(4.0));
                }
            }
            false => {
                spark.stop();
            }
        };
    }
}
