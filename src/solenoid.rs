use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::Primitive::Void;
use jni::signature::ReturnType;
use jni::sys::jboolean;
use crate::java;

pub struct Solenoid {
    instance: GlobalRef
}

pub enum ModuleType {
    Rev,
    CTRE
}

impl Solenoid {
    pub fn new(module_type: ModuleType, channel: i32) -> Self {
        let module_type_java = match module_type {
            ModuleType::Rev => call_static!(
                "frc/robot/Wrapper",
                "revPH",
                "()Ledu/wpi/first/wpilibj/PneumaticsModuleType;",
                &Vec::new(),
                ReturnType::Object
            ).l().unwrap(),
            ModuleType::CTRE => call_static!(
                "frc/robot/Wrapper",
                "ctrePCM",
                "()Ledu/wpi/first/wpilibj/PneumaticsModuleType;",
                &Vec::new(),
                ReturnType::Object
            ).l().unwrap(),
        };

        let instance = create!(
            "edu/wpi/first/wpilibj/Solenoid",
            "(Ledu/wpi/first/wpilibj/PneumaticsModuleType;I)V",
            &[JValue::Object(&JObject::from(module_type_java)).as_jni(), JValue::Int(channel).as_jni()]
        );

        Self {
            instance
        }
    }

    pub fn set(&self, engaged: bool) {
        call!(
            &self.instance,
            "edu/wpi/first/wpilibj/Solenoid",
            "set",
            "(Z)V",
            &[JValue::Bool(jboolean::from(engaged)).as_jni()],
            ReturnType::Primitive(Void)
        );
    }
}