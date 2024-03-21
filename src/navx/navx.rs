use j4rs::{Instance, Jvm};
use jni::{objects::JObject, signature::{Primitive, ReturnType}};

use crate::call::{call, call_static};

pub struct NavX<'local> {
    instance: JObject<'local>,
}

impl<'local> NavX<'local> {
    pub fn new() -> Self {
        let instance = call_static!(
            "frc/robot/Wrapper",
            "createAHRS",
            "()Lcom/kauailabs/navx/frc/AHRS;",
            &Vec::new(),
            ReturnType::Object
        ).l().unwrap();

        Self {
            instance
        }
    }

    pub fn get_angle(&self) -> f64 {
        call!(
            &self.instance,
            "com/kauailabs/navx/frc/AHRS",
            "getAngle",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn reset_angle(&self) {
        call!(
            &self.instance,
            "com/kauailabs/navx/frc/AHRS",
            "reset",
            "()V",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Void)
        ).v().unwrap()
    }
}
