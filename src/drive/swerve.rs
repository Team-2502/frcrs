/*use crate::{Encoder, Motor};

pub struct Motors {
    fl_drive: dyn Motor,
    fl_turn: dyn Motor,
    fr_drive: dyn Motor,
    fr_turn: dyn Motor,
    bl_drive: dyn Motor,
    bl_turn: dyn Motor,
    br_drive: dyn Motor,
    br_turn: dyn Motor,
}

pub struct Encoders {
    fl: dyn Encoder,
    fr: dyn Encoder,
    bl: dyn Encoder,
    br: dyn Encoder,
}

impl Encoders {
    pub fn new(fl: Box<dyn Encoder>, fr: Box<dyn Encoder>, bl: Box<dyn Encoder>, br: Box<dyn Encoder>) -> Box<Self> {
        Box::new(Self {
            fl,
            fr,
            bl,
            br
        })
    }
}

impl Motors {
    pub fn new(fl_drive: Box<dyn Motor>, fr_drive: Box<dyn Motor>, bl_drive: Box<dyn Motor>, br_drive: Box<dyn Motor>,
               fl_turn: Box<dyn Motor>, fr_turn: Box<dyn Motor>, bl_turn: Box<dyn Motor>, br_turn: Box<dyn Motor>) -> Box<Self> {
        Box::new(Self {
            fl_drive,
            fl_turn,
            fr_drive,
            fr_turn,
            bl_drive,
            bl_turn,
            br_drive,
            br_turn,
        })
    }
}

pub struct SwerveBuilder {
    motors: Motors,
    encoders: Encoders,
}

impl SwerveBuilder {
    pub fn new(fl_drive: Box<dyn Motor>, fr_drive: Box<dyn Motor>, bl_drive: Box<dyn Motor>, br_drive: Box<dyn Motor>,
               fl_turn: Box<dyn Motor>, fr_turn: Box<dyn Motor>, bl_turn: Box<dyn Motor>, br_turn: Box<dyn Motor>,
    fl_encoder: Box<dyn Encoder>, fr_encoder: Box<dyn Encoder>, bl_encoder: Box<dyn Encoder>, br_encoder: Box<dyn Encoder>) -> Box<Self> {

        Box::new(Self {
            motors: *Motors::new(fl_drive, fl_turn, fr_drive, fr_turn, bl_drive, bl_turn, br_drive, br_turn),
            encoders: *Encoders::new(fl_encoder, fr_encoder, bl_encoder, br_encoder)
        })
    }

    pub fn set_speeds(&self) {

    }
}*/

use std::f64::consts::PI;
use uom::num_traits::{Pow, PrimInt};
use crate::networktables::SmartDashboard;

pub struct WheelSpeeds {
    pub ws1: f64,
    pub ws2: f64,
    pub ws3: f64,
    pub ws4: f64,

    pub wa1: f64,
    pub wa2: f64,
    pub wa3: f64,
    pub wa4: f64,
}

pub struct Swerve;

impl Swerve {
    pub fn calculate(fwd: f64, str: f64, rot: f64, angle: f64) -> WheelSpeeds {
        let temp = (fwd * angle.cos()) + (str * angle.sin());
        let new_str = (-fwd * angle.sin()) + (str * angle.cos());
        let new_fwd = temp;

        let wheelbase = 32.;
        let track_width = 32.;
        let r = f64::sqrt((wheelbase.pow(2) + track_width.pow(2)) as f64);

        let a: f64 = new_str - rot * (wheelbase / r);
        let b: f64 = new_str + rot * (wheelbase / r);
        let c: f64 = new_fwd - rot * (track_width / r);
        let d: f64 = new_fwd + rot * (track_width / r);

        let mut ws1 = f64::sqrt(b.pow(2) + c.pow(2));
        let mut ws2 = f64::sqrt(b.pow(2) + d.pow(2));
        let mut ws3 = f64::sqrt(a.pow(2) + d.pow(2));
        let mut ws4 = f64::sqrt(a.pow(2) + c.pow(2));

        let wa1 = b.atan2(c) * (180. / PI);
        let wa2 = b.atan2(d) * (180. / PI);
        let wa3 = a.atan2(d) * (180. / PI);
        let wa4 = a.atan2(c) * (180. / PI);

        let mut max = ws1;
        if ws2 > max { max = ws2; } else if ws3 > max { max = ws3; } else if ws4 > max { max = ws4 }
        if max > 1. {
            ws1 /= max;
            ws2 /= max;
            ws3 /= max;
            ws4 /= max;
        }

        /*SmartDashboard::put_number("fl turn".to_owned(), wa2);
        SmartDashboard::put_number("fr turn".to_owned(), wa1);
        SmartDashboard::put_number("bl turn".to_owned(), wa3);
        SmartDashboard::put_number("br turn".to_owned(), wa4);*/

        WheelSpeeds {
            ws1,
            ws2,
            ws3,
            ws4,

            wa1,
            wa2,
            wa3,
            wa4
        }
    }

    pub fn optimize(target_speed: f64, target_angle: f64, current_angle: f64) -> (f64, f64) {
        let mut target_angle = Self::place_in_appropriate_0_to_360_scope(current_angle, target_angle);
        //println!("{}", target_angle);

        let delta = target_angle - current_angle;
        let offset = if delta > 0. {
            180.
        } else { -180. };

        return if delta.abs() > 90. {
            (-target_speed, target_angle - offset)
        } else {
            (target_speed, target_angle - offset)
        }
    }

    pub fn place_in_appropriate_0_to_360_scope(scope_ref: f64, new_angle: f64) -> f64 {
        let mut lower_bound = 0.;
        let mut upper_bound = 0.;
        let mut _new_angle = new_angle;
        let lower_offset = scope_ref % 360.;

        if lower_offset >= 0. {
            lower_bound = scope_ref - lower_offset;
            upper_bound = scope_ref + (360. - lower_offset);
        } else {
            upper_bound = scope_ref - lower_offset;
            lower_bound = scope_ref - (360. + lower_offset);
        }
        while _new_angle < lower_bound {
            _new_angle += 360.;
        }
        while new_angle > upper_bound {
            _new_angle -= 360.;
        }
        if _new_angle - scope_ref > 180. {
            _new_angle -= 360.;
        } else if _new_angle - scope_ref < -180. {
            _new_angle += 360.;
        }

        _new_angle
    }
}

pub trait ToTalonEncoder {
    fn talon_encoder_ticks(&self) -> f64;
    fn from_talon_encoder_ticks(&self) -> f64;
}

impl ToTalonEncoder for f64 {
    fn talon_encoder_ticks(&self) -> f64 {
        self / ((360.) / (2048. * 12.8))
    }

    fn from_talon_encoder_ticks(&self) -> f64 {
        self * ((360.) / (2048. * 12.8))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::drive::{Swerve, ToTalonEncoder};

    #[test]
    fn place_in_scope() {
        let angle = Swerve::place_in_appropriate_0_to_360_scope(720., 90.);

        assert_eq!(810., angle);
    }

    #[test]
    fn opt_invert() {
        let angle = Swerve::optimize(1., 180., 0.);

        let in_range = Swerve::place_in_appropriate_0_to_360_scope(0., angle.1);
        assert_eq!((angle.0, in_range), (-1., 0.));
    }

    #[test]
    fn calc() {
        let wheel_speeds = Swerve::calculate(0.5, 0., 0., 0.);

        assert_eq!((wheel_speeds.ws1, wheel_speeds.wa1), (0.5, 0.));
    }

    #[test]
    fn opt() {
        let wheel_speeds = Swerve::calculate(0.5, 0., 0., 0.);

        let angle1 = Swerve::optimize(wheel_speeds.ws1, wheel_speeds.wa1, 0.);

        assert_eq!((angle1.0, angle1.1), (0., 10.));
    }

    #[test]
    fn opt_invert_high() {
        let angle = Swerve::optimize(1., 180., 725.);

        let in_range = Swerve::place_in_appropriate_0_to_360_scope(725., angle.1);
        assert_eq!((angle.0, in_range), (-1., 720.));
    }

    /*#[test]
    fn calculate_speed() {
        let start = Instant::now();

        let wheel_speeds = Swerve::calculate(0.5, 0.5,0.5, 142.15);

        let fr_speeds = Swerve::optimize(
            wheel_speeds.ws1, wheel_speeds.wa1, 2151.
        );

        let fl_speeds = Swerve::optimize(
            wheel_speeds.ws2, wheel_speeds.wa2, 2615.
        );

        let bl_speeds = Swerve::optimize(
            wheel_speeds.ws3, wheel_speeds.wa3, 2725.
        );

        let br_speeds = Swerve::optimize(
            wheel_speeds.ws4, wheel_speeds.wa4, 2409.
        );

        let fr_turn_pos =  Swerve::place_in_appropriate_0_to_360_scope(
            2151., fr_speeds.1).talon_encoder_ticks();

        let fl_turn_pos = Swerve::place_in_appropriate_0_to_360_scope(
            2615., fl_speeds.1).talon_encoder_ticks();

        let bl_turn_pos = Swerve::place_in_appropriate_0_to_360_scope(
            2725., bl_speeds.1).talon_encoder_ticks();

        let br_turn_pos = Swerve::place_in_appropriate_0_to_360_scope(
            2409., br_speeds.1).talon_encoder_ticks();

        println!("{}μs", start.elapsed().as_micros());
    }*/
}