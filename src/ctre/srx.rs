use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::Primitive::Void;
use jni::signature::ReturnType;
use crate::call::{call, call_static, create};
use crate::java;

pub struct SRX {
    instance: GlobalRef
}

impl SRX {
    pub fn new(id: i32) -> Self {
        let instance = create!(
            "com.ctre.phoenix.motorcontrol.can.TalonSRX",
            "(I)V",
            &[JValue::Int(id).as_jni()]
        );

        Self {
            instance
        }
    }

    pub fn set(&self, value: f64) {
        let string = java().new_string("PercentOutput").unwrap();

        let control = call_static!(
            "com.ctre.phoenix.motorcontrol.TalonSRXControlMode",
            "valueOf",
            "(Ljava/lang/String;)Lcom/ctre/phoenix/motorcontrol/TalonSRXControlMode;",
            &[JValue::Object(&JObject::from(string)).as_jni()],
            ReturnType::Object
        ).l().unwrap();

        call!(
            &self.instance,
            "com.ctre.phoenix.motorcontrol.can.TalonSRX",
            "set",
            "(Lcom/ctre/phoenix/motorcontrol/TalonSRXControlMode;DLcom/ctre/phoenix/motorcontrol/DemandType;D)V",
            &[JValue::Object(&JObject::from(control)).as_jni(), JValue::Double(value).as_jni()],
            ReturnType::Primitive(Void)
        );
    }
}