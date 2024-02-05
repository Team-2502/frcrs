pub mod ctre;
pub mod input;
pub mod networktables;
pub mod rev;
pub mod robot;
pub mod navx;
pub mod drive;

use input::Joystick;
pub use j4rs_derive::call_from_java;
use networktables::SmartDashboard;
use rev::SparkMax;

#[macro_use]
extern crate uom;

use crate::rev::ControlType::Position;
use crate::rev::{IdleMode, MotorType, Spark, SparkPIDController};
use j4rs::prelude::*;
use std::convert::TryFrom;
use std::ops::Range;
use std::thread::sleep;
use std::time::Duration;
use j4rs::InvocationArg;
use uom::si::angle::degree;
use uom::si::angle::revolution;
use uom::si::f64::*;
use crate::ctre::{CanCoder, ControlMode, Kraken};
use crate::ctre::TalonInvertType::CounterClockwise;
use crate::drive::{Swerve, ToTalonEncoder};
use crate::navx::NavX;
use crate::rev::MotorType::Brushless;
/*
#[call_from_java("frc.robot.Main.rustentry")]
fn entrypoint() {
    observe_user_program_starting();

    if !init_hal() {
        panic!("Failed to init HAL")
    }

    hal_report(2, 3, 0, "2023.3.1".to_string());

    SmartDashboard::init();

    let navx = NavX::new();

    let encoder = CanCoder::new(9, Some("can0".to_owned()));

    let driver_right = Joystick::new(0);
    let driver_left = Joystick::new(1);

    let fl_drive = Talon::new(1,  Some("can0".to_owned()));
    let fl_turn = Talon::new(2,  Some("can0".to_owned()));

    let fr_drive = Talon::new(4,  Some("can0".to_owned()));
    let fr_turn = Talon::new(5,  Some("can0".to_owned()));

    let bl_drive = Talon::new(7,  Some("can0".to_owned()));
    let bl_turn = Talon::new(8,  Some("can0".to_owned()));

    let br_drive = Talon::new(10,  Some("can0".to_owned()));
    let br_turn = Talon::new(11,  Some("can0".to_owned()));

        loop {
            refresh_data();

            match is_teleop() {
                true => {
                    SmartDashboard::put_number("angle".to_owned(), navx.get_angle());
                    SmartDashboard::put_number("br angle".to_owned(), encoder.get());

                    let wheel_speeds = Swerve::calculate(
                        driver_left.get_y(), driver_left.get_x(), driver_right.get_z(), navx.get_angle());

                    let fr_speeds = Swerve::optimize(
                        wheel_speeds.ws1, wheel_speeds.wa1, fr_drive.get().from_talon_encoder_ticks()
                    );

                    let fl_speeds = Swerve::optimize(
                        wheel_speeds.ws2, wheel_speeds.wa2, fl_drive.get().from_talon_encoder_ticks()
                    );

                    let bl_speeds = Swerve::optimize(
                        wheel_speeds.ws3, wheel_speeds.wa3, bl_drive.get().from_talon_encoder_ticks()
                    );

                    let br_speeds = Swerve::optimize(
                        wheel_speeds.ws4, wheel_speeds.wa4, br_drive.get().from_talon_encoder_ticks()
                    );

                    /*SmartDashboard::put_number("fl spd".to_owned(), fl_speeds.0);
                    SmartDashboard::put_number("fr spd".to_owned(), fr_speeds.0);
                    SmartDashboard::put_number("bl spd".to_owned(), bl_speeds.0);
                    SmartDashboard::put_number("br spd".to_owned(), br_speeds.0);*/

                    /*let fr_turn_pos =  Swerve::place_in_appropriate_0_to_360_scope(
                        fr_turn.get(), fr_speeds.1) / ((360.) / (2048. * 12.8));

                    let fl_turn_pos = Swerve::place_in_appropriate_0_to_360_scope(
                        fl_turn.get(), fl_speeds.1) / ((360.) / (2048. * 12.8));

                    let bl_turn_pos = Swerve::place_in_appropriate_0_to_360_scope(
                        bl_turn.get(), bl_speeds.1) / ((360.) / (2048. * 12.8));

                    let br_turn_pos = Swerve::place_in_appropriate_0_to_360_scope(
                        br_turn.get(), br_speeds.1) / ((360.) / (2048. * 12.8));*/

                    /*SmartDashboard::put_number("fl opt".to_owned(), fl_turn_pos);
                    SmartDashboard::put_number("fr opt".to_owned(), fr_turn_pos);
                    SmartDashboard::put_number("bl opt".to_owned(), bl_turn_pos);
                    SmartDashboard::put_number("br opt".to_owned(), br_turn_pos);*/

                    fr_drive.set(ControlMode::Percent, fr_speeds.0);
                    fl_drive.set(ControlMode::Percent, fl_speeds.0);
                    bl_drive.set(ControlMode::Percent, bl_speeds.0);
                    br_drive.set(ControlMode::Percent, br_speeds.0);

                    fr_turn.set(ControlMode::Position, fr_speeds.1.talon_encoder_ticks());
                    fl_turn.set(ControlMode::Position, fl_speeds.1.talon_encoder_ticks());
                    bl_turn.set(ControlMode::Position, bl_speeds.1.talon_encoder_ticks());
                    br_turn.set(ControlMode::Position, br_speeds.1.talon_encoder_ticks());

                    /*fr_drive.set(ControlMode::Percent, wheel_speeds.ws2);
                    fl_drive.set(ControlMode::Percent, wheel_speeds.ws1);
                    bl_drive.set(ControlMode::Percent, wheel_speeds.ws4);
                    br_drive.set(ControlMode::Percent, wheel_speeds.ws3);

                    fr_turn.set(ControlMode::Position, wheel_speeds.wa2 / ((360.) / (2048. * 12.8)));
                    fl_turn.set(ControlMode::Position, wheel_speeds.wa1 / ((360.) / (2048. * 12.8)));
                    bl_turn.set(ControlMode::Position, wheel_speeds.wa4 / ((360.) / (2048. * 12.8)));
                    br_turn.set(ControlMode::Position, wheel_speeds.wa3 / ((360.) / (2048. * 12.8)));*/
                },
                false => {
                    fl_turn.stop();
                    fl_drive.stop();

                    fr_turn.stop();
                    fr_drive.stop();

                    bl_turn.stop();
                    bl_drive.stop();

                    br_turn.stop();
                    br_drive.stop();
                }
        }
    }
}
*/
/// Map x (within from) to the same relative spot in to
fn deadzone(input: f64, from_range: &Range<f64>, to_range: &Range<f64>) -> f64 {
    let neg = input < 0.0;
    let input = input.abs();
    let from_len = from_range.end - from_range.start;
    let to_len = to_range.end - to_range.start;
    let standard = (input - from_range.start) / from_len;
    let mut out = (standard * to_len) + to_range.start;
    if input < from_range.start { out = 0.0 };
    if neg {
        -out
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::deadzone;
    #[test]
    fn deadzone_test() {
        let result = deadzone(0.5, &(0.0..1.0), &(0.0..2.0));
        assert_eq!(result, 1.0);
    }

    #[test]
    fn deadzone_reverse_test() {
        let result = deadzone(-0.5, &(0.0..1.0), &(0.0..2.0));
        assert_eq!(result, -1.0);
    }

    #[test]
    fn deadzone_reverse_test_2() {
        let result = deadzone(-0.75, &(0.5..1.0), &(0.0..2.0));
        assert_eq!(result, -1.0);
    }

    #[test]
    fn deadzone_test_2() {
        let result = deadzone(0.75, &(0.5..1.0), &(0.0..2.0));
        assert_eq!(result, 1.0);
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

pub fn init_hal() -> bool {
    let jvm = Jvm::attach_thread().unwrap();

    let value: bool = jvm
        .to_rust(
            jvm.invoke_static(
                "edu.wpi.first.hal.HAL",
                "initialize",
                &[InvocationArg::try_from(500).unwrap().into_primitive().unwrap(),
                    InvocationArg::try_from(0).unwrap().into_primitive().unwrap(),],
            )
                .unwrap(),
        )
        .unwrap();
    value
}

pub fn hal_report(resource: i32, instance_number: i32, context: i32, feature: String) {
    let jvm = Jvm::attach_thread().unwrap();

    jvm.invoke_static(
        "edu.wpi.first.hal.HAL",
        "report",
        &[
            InvocationArg::try_from(resource).unwrap().into_primitive().unwrap(),
            InvocationArg::try_from(instance_number).unwrap().into_primitive().unwrap(),
            InvocationArg::try_from(context).unwrap().into_primitive().unwrap(),
            InvocationArg::try_from(feature).unwrap(),
        ],
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

/*pub enum Keyword {
    Auto,
    Teleop,
    Practice,
    Test
}

pub fn get_keyword() -> Keyword {

}*/

pub trait Motor {
    fn set(&self, value: f64);
    fn stop(&self);
}

pub trait Encoder {
    fn get(&self) -> f64;
}
