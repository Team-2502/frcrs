use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::{Primitive, ReturnType};
use crate::call::{call, call_static, create};
use crate::java;

pub struct CanCoder {
    instance: GlobalRef
}

impl CanCoder {
    pub fn new(id: i32, can_loop: Option<String>) -> Self {
        let string = java().new_string(can_loop.unwrap_or("rio".to_string())).unwrap();

        let instance = create!(
            "com/ctre/phoenix6/hardware/core/CoreCANcoder",
            "(ILjava/lang/String;)V",
            &[JValue::Int(id).as_jni(),
                JValue::Object(&JObject::from_raw(string.into_raw())).as_jni()
            ]
        );

        let instance = java().new_global_ref(instance).unwrap();

        Self {
            instance
        }
    }

    /*pub fn get(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "com/ctre/phoenix/sensors/CANCoder",
            "getPosition",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }*/

    pub fn get_absolute(&self) -> f64 {
        let status_signal = call!(
            self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/core/CoreCANcoder",
            "getAbsolutePosition",
            "()Lcom/ctre/phoenix6/StatusSignal;",
            &Vec::new(),
            ReturnType::Object
        ).l().unwrap();

        call_static!(
            "frc/robot/Wrapper",
            "getValue",
            "(Lcom/ctre/phoenix6/StatusSignal;)D",
            &[JValue::Object(&status_signal).as_jni()],
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }
}