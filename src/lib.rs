pub mod ctre;
pub mod input;
pub mod networktables;
pub mod rev;
pub mod robot;
pub mod navx;
pub mod drive;
pub mod dio;
#[macro_use]
pub mod call;

use input::Joystick;
pub use j4rs_derive::call_from_java;
use jni::objects::{JObject, JString, JValue, JValueGen};
use jni::signature::Primitive;
use jni::strings::JNIString;
use jni::sys::jint;
use jni::{InitArgsBuilder, JNIEnv, JNIVersion, JavaVM};
use lazy_static::lazy_static;
use networktables::SmartDashboard;

#[macro_use]
extern crate uom;

use crate::rev::ControlType::Position;
use crate::rev::{IdleMode, MotorType, Spark, };
use j4rs::prelude::*;
use std::convert::TryFrom;
use std::ops::Range;
use std::thread::sleep;
use std::time::Duration;
use j4rs::InvocationArg;
use uom::si::angle::degree;
use uom::si::angle::revolution;
use uom::si::f64::*;
use crate::ctre::{CanCoder, ControlMode};
use crate::ctre::TalonInvertType::CounterClockwise;
use crate::drive::{Swerve, ToTalonEncoder};
use crate::navx::NavX;
use crate::rev::MotorType::Brushless;


fn create_jvm() -> JavaVM{
    // set JAVA_HOME to /usr/local/frc/JRE/bin/
    let jvm_args = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        .option("-XX:+UseSerialGC")
        .option("-Djava.lang.invoke.stringConcat=BC_SB")
        .option("-Djava.library.path=/usr/local/frc/third-party/lib")
        .option("-Djava.class.path=/home/lvuser/javastub.jar")
        .build().unwrap();

    let jvm = JavaVM::with_libjvm(jvm_args, || Ok("/usr/local/frc/JRE/lib/client/libjvm.so")).unwrap();
    jvm.attach_current_thread_as_daemon().unwrap();
    jvm
}

lazy_static!{
    static ref  JAVA: JavaVM = create_jvm();
}

fn java() -> JNIEnv<'static> {
    JAVA.attach_current_thread_permanently().unwrap()
}

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
    // Show "robot code" on driver's station
    call_static!(
        "edu/wpi/first/hal/DriverStationJNI",
        "observeUserProgramStarting",
        "()V",
        &Vec::new(),
        jni::signature::ReturnType::Primitive(Primitive::Void)
    );
}

pub fn refresh_data() {
    call_static!(
        "edu/wpi/first/wpilibj/DriverStation",
        "refreshData",
        "()V",
        &Vec::new(),
        jni::signature::ReturnType::Primitive(Primitive::Void)
    );
}

pub fn init_hal() -> bool {
    call_static!(
		"edu/wpi/first/hal/HAL",
		"initialize",
		"(II)Z",
		&[JValue::Int(500).as_jni(),
          JValue::Int(1).as_jni()],
        jni::signature::ReturnType::Primitive(Primitive::Boolean)
    ).z().unwrap()
}

pub fn hal_report(resource: i32, instance_number: i32, context: i32, feature: String) {
    let string = java().new_string(feature).unwrap();
    call_static!(
		"edu/wpi/first/hal/HAL",
		"report",
		"(IIILjava/lang/String;)I",
		&[JValue::Int(resource).as_jni(),
		  JValue::Int(instance_number).as_jni(),
          JValue::Int(context).as_jni(),
          JValue::Object(&JObject::from_raw(string.into_raw())).as_jni()
        ],
        jni::signature::ReturnType::Primitive(Primitive::Int)
    ).i().unwrap();
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
    let station = call_static!(
		"frc/robot/Wrapper",
		"getAllianceStation",
		"()I",
        &Vec::new(),
        jni::signature::ReturnType::Primitive(Primitive::Int)
    ).i().unwrap();

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
