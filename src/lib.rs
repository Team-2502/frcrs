pub mod ctre;
pub mod input;
pub mod rev;
pub mod robot;
pub mod navx;
pub mod drive;
pub mod dio;


use hal_sys::HAL_Bool;
pub use j4rs_derive::call_from_java;




#[macro_use]
extern crate uom;



use j4rs::prelude::*;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::ops::Range;


use j4rs::InvocationArg;











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
pub fn deadzone(input: f64, from_range: &Range<f64>, to_range: &Range<f64>) -> f64 {
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
    unsafe {hal_sys::HAL_ObserveUserProgramStarting()}
}

pub fn refresh_data() {
    unsafe {hal_sys::HAL_RefreshDSData()};
}

pub fn init_hal() -> bool {
   let ret =  unsafe{hal_sys::HAL_Initialize(500, 1)}; // force kill
   0 == ret
}

pub fn hal_report(resource: i32, instance_number: i32, context: i32, feature: String) {
    let feature = CString::new(feature).unwrap();
    unsafe{hal_sys::HAL_Report(resource, instance_number, context, feature.as_ptr())};
}

pub struct AllianceStation(u8);

impl AllianceStation {
   pub fn red(&self) -> bool {
       match self.0 {
           1 | 2 | 3 => true,
           _ => false,
       }
   }
   pub fn blue(&self) -> bool {
       match self.0 {
           4 | 5 | 6 => true,
           _ => false,
       }
   }
}

pub fn alliance_station() -> AllianceStation {
    let mut station = 0;

    let _ret = unsafe{hal_sys::HAL_GetAllianceStation(&mut station)};

    AllianceStation(station as u8)
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
