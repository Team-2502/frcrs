use j4rs::Jvm;
use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::{Primitive, ReturnType};
use crate::java;

pub struct Limelight {
    pub (crate) instance: GlobalRef
}

impl Limelight {
    pub fn test() -> f64 {
        call_static!(
            "frc/robot/Wrapper",
            "limelightGetTX",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn get_tx(name: &str) -> f64 {
        let string = java().new_string(name).unwrap();

        call_static!(
            "frc/robot/LimelightHelpers",
            "getTX",
            "(Ljava/lang/String;)D",
            &[JValue::Object(&JObject::from_raw(string.into_raw())).as_jni()],
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn get_ty(name: &str) -> f64 {
        let string = java().new_string(name).unwrap();

        call_static!(
            "frc/robot/LimelightHelpers",
            "getTY",
            "(Ljava/lang/String;)D",
            &[JValue::Object(&JObject::from_raw(string.into_raw())).as_jni()],
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn get_ta(name: &str) -> f64 {
        let string = java().new_string(name).unwrap();

        call_static!(
            "frc/robot/LimelightHelpers",
            "getTA",
            "(Ljava/lang/String;)D",
            &[JValue::Object(&JObject::from_raw(string.into_raw())).as_jni()],
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }
}