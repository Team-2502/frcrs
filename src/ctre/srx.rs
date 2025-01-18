use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::Primitive::Void;
use jni::signature::ReturnType;
use crate::call::{call, call_static, create};

pub struct SRX {
    instance: GlobalRef
}

impl SRX {
    pub fn new(id: i32) -> Self {
        let instance = create!(
            "com/ctre/phoenix/motorcontrol/can/TalonSRX",
            "(I)V",
            &[JValue::Int(id).as_jni()]
        );

        Self {
            instance
        }
    }

    pub fn set(&self, value: f64) {
        let control = call_static!(
            "frc/robot/Wrapper",
            "srxPercentOut",
            "()Lcom/ctre/phoenix/motorcontrol/TalonSRXControlMode;",
            &Vec::new(),
            ReturnType::Object
        ).l().unwrap();

        call!(
            &self.instance,
            "com/ctre/phoenix/motorcontrol/can/TalonSRX",
            "set",
            "(Lcom/ctre/phoenix/motorcontrol/TalonSRXControlMode;D)V",
            &[JValue::Object(&JObject::from(control)).as_jni(), JValue::Double(value).as_jni()],
            ReturnType::Primitive(Void)
        );
    }
}