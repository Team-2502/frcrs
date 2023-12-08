pub mod ctre;
pub mod input;
pub mod networktables;
pub mod rev;
pub mod robot;

use input::Joystick;
pub use j4rs_derive::call_from_java;
use networktables::SmartDashboard;
use rev::SparkMax;

use crate::rev::ControlType::Position;
use crate::rev::{IdleMode, MotorType, Spark, SparkPIDController};
use j4rs::prelude::*;
use std::convert::TryFrom;
use uom::si::angle::degree;
use uom::si::angle::revolution;
use uom::si::f64::*;
use crate::ctre::Talon;
use crate::ctre::TalonInvertType::CounterClockwise;
use crate::rev::MotorType::Brushless;

#[call_from_java("frc.robot.Main.rustentry")]
fn entrypoint() {
    observe_user_program_starting();

    // Drivetrain
    let fl_drive = Talon::new(3, None);
    let fr_drive = Talon::new(1, None);
    let bl_drive = Talon::new(4, None);
    let br_drive = Talon::new(2, None);

    //&fl_drive.set_inverted(CounterClockwise);
    //&fr_drive.set_inverted(CounterClockwise);

    //bl_drive.follow(fl_drive);
    //br_drive.follow(fr_drive);

    // Intake
    let intake = Spark::new(8, Brushless);

    // Hopper
    let hopper_right = Spark::new(4, Brushless);
    let hopper_left = Spark::new(3, Brushless);
    let hopper_bottom = Spark::new(5, Brushless);

    // Shooter
    let shooter_feed = Spark::new(6, Brushless);
    let shooter_left = Spark::new(12, Brushless);
    let shooter_right = Spark::new(2, Brushless);

    //shooter_left.follow(shooter_right);

    /*let pid = shooter_right.get_pid();
    pid.set_p(0.0008);
    pid.set_i(0.0);
    pid.set_d(0.0);
    pid.set_i_zone(0.);
    pid.set_ff(0.00019);
    pid.set_output_range(-0.1, 1.);*/

    let left_joystick = Joystick::new(1);
    let right_joystick = Joystick::new(0);
    let op_joystick = Joystick::new(2);

    loop {
        let teleop = is_teleop();

        refresh_data();

        match teleop {
            true => {
                // Drive
                fl_drive.set(left_joystick.get_y() * 0.25);
                fr_drive.set(right_joystick.get_y() * 0.25);

                bl_drive.set(-(left_joystick.get_y() * 0.25));
                br_drive.set(-(right_joystick.get_y() * 0.25));

                // Intake
                if right_joystick.get(2) { intake.set(0.85); }
                if left_joystick.get(2) { intake.set(-1.); }

                // Hopper
                if op_joystick.get(3) {
                    hopper_left.set(1.);
                    hopper_right.set(0.25);
                    hopper_bottom.set(1.);
                }

                if op_joystick.get(4) {
                    hopper_left.set(-1.);
                    hopper_right.set(-1.);
                    hopper_bottom.set(-1.);
                }

                // Shooter
                if op_joystick.get(2) {
                    shooter_feed.set(1.);
                }

                if op_joystick.get(5) {
                    shooter_right.set((op_joystick.get_throttle() + 1f64) / 2f64);
                    shooter_left.set(-((op_joystick.get_throttle() + 1f64) / 2f64));
                }
            }
            false => {
                fl_drive.stop();
                fr_drive.stop();
                bl_drive.stop();
                br_drive.stop();

                intake.stop();

                hopper_bottom.stop();
                hopper_right.stop();
                hopper_left.stop();

                shooter_right.stop();
                shooter_left.stop();
                shooter_feed.stop();
            }
        };
    }
}

pub fn observe_user_program_starting() {
    let jvm = Jvm::attach_thread().unwrap();

    // Show "robot code" on driver's station
    jvm.invoke_static(
        "edu.wpi.first.hal.DriverStationJNI",
        "observeUserProgramStarting",
        &Vec::new(),
    )
        .unwrap();
}

pub fn refresh_data() {
    let jvm = Jvm::attach_thread().unwrap();

    jvm.invoke_static(
        "edu.wpi.first.wpilibj.DriverStation",
        "refreshData",
        &Vec::new(),
    ).unwrap();
}

pub fn is_teleop() -> bool {
    let jvm = Jvm::attach_thread().unwrap();

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

    teleop
}