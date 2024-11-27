use auto_jni::create;
use auto_jni::jni::objects::{GlobalRef, JObject, JValue};
use auto_jni::once_cell;
use auto_jni::jni;
use crate::{com_ctre_phoenix6_hardware_TalonFX, java};

pub struct TalonFX {
    motor: com_ctre_phoenix6_hardware_TalonFX,
}

impl TalonFX {
    pub fn new(id: i32, bus: Option<String>) -> Self {
        let string = java().new_string(bus.unwrap_or("rio".to_string())).unwrap();

        Self {
            motor: com_ctre_phoenix6_hardware_TalonFX::new_2(id, JObject::from(string)).unwrap()
        }
    }

    pub fn set(&self, value: f64) {
        com_ctre_phoenix6_hardware_TalonFX::set(&self.motor.inner(), value).unwrap()
    }

    pub fn stop(&self) {
        com_ctre_phoenix6_hardware_TalonFX::stopMotor(&self.motor.inner()).unwrap()
    }
}