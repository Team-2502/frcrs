use crate::call::{call, call_static, create};
use crate::java;
use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::{Primitive, ReturnType};
use jni::sys::jboolean;
use uom::si::f64::Length;
use uom::si::length::meter;

pub struct CanRange {
    instance: GlobalRef,
}

impl CanRange {
    pub fn new(id: i32, can_loop: Option<String>) -> Self {
        let string = java()
            .new_string(can_loop.unwrap_or("rio".to_string()))
            .unwrap();

        let instance = create!(
            "com/ctre/phoenix6/hardware/core/CoreCANrange",
            "(ILjava/lang/String;)V",
            &[
                JValue::Int(id).as_jni(),
                JValue::Object(&JObject::from_raw(string.into_raw())).as_jni()
            ]
        );

        let instance = java().new_global_ref(instance).unwrap();

        Self { instance }
    }
    pub fn get_distance(&self) -> Length {
        let status_signal = call!(
            self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/core/CoreCANrange",
            "getDistance",
            "(Z)Lcom/ctre/phoenix6/StatusSignal;",
            &[JValue::Bool(jboolean::from(true)).as_jni()],
            ReturnType::Object
        )
        .l()
        .unwrap();

        let dist_meters: f64 = call_static!(
            "frc/robot/Wrapper",
            "getValue",
            "(Lcom/ctre/phoenix6/StatusSignal;)D",
            &[JValue::Object(&status_signal).as_jni()],
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap();

        Length::new::<meter>(dist_meters)
    }
}
