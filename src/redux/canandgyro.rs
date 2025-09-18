use crate::call::{call, create};
use crate::java;
use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::{Primitive, ReturnType};
use nalgebra::{Rotation3, Vector3};

pub struct CanAndGyro {
    instance: GlobalRef,
}

impl CanAndGyro {
    pub fn new(id: i32) -> Self {
        let instance = create!(
            "com/reduxrobotics/sensors/canandgyro/Canandgyro",
            "(I)V",
            &[JValue::Int(id).as_jni()]
        );

        Self {
            instance: java().new_global_ref(instance).unwrap(),
        }
    }

    pub fn get_angle(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "com/reduxrobotics/sensors/canandgyro/Canandgyro",
            "getMultiturnYaw",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap()
    }
}
