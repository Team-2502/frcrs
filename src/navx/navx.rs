use jni::{
    objects::GlobalRef,
    signature::{Primitive, ReturnType},
};

use crate::{
    call::{call, call_static},
    java,
};

pub struct NavX {
    instance: GlobalRef,
}

impl NavX {
    pub fn new() -> Self {
        let instance = call_static!(
            "frc/robot/Wrapper",
            "createAHRS",
            //"()L",
            "()Lcom/studica/frc/AHRS;",
            &Vec::new(),
            ReturnType::Object
        )
        .l()
        .unwrap();

        let instance = java().new_global_ref(instance).unwrap();

        Self { instance }
    }

    pub fn get_angle(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "com/kauailabs/navx/frc/AHRS",
            "getAngle",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap()
    }

    pub fn reset_angle(&self) {
        call!(
            self.instance.as_obj(),
            "com/kauailabs/navx/frc/AHRS",
            "reset",
            "()V",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Void)
        )
        .v()
        .unwrap()
    }
}
