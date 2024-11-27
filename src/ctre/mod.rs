use auto_jni::create;
use auto_jni::jni::objects::{GlobalRef, JObject, JValue};
use auto_jni::once_cell;
use auto_jni::jni;
use crate::{com_ctre_phoenix6_hardware_TalonFX, java};

pub struct TalonFX {
    motor: com_ctre_phoenix6_hardware_TalonFX,
}

fn create_talon(id: i32, can_bus: Option<String>) -> GlobalRef {
    let string = java().new_string(can_bus.unwrap_or("rio".to_string())).unwrap();

    let instance = create!(
            "com/ctre/phoenix6/hardware/TalonFX",
            "(ILjava/lang/String;)V",
            &[JValue::Int(id).as_jni(),
                JValue::Object(&JObject::from_raw(string.into_raw())).as_jni()
            ]
        );

    java().new_global_ref(instance).unwrap()
}

impl TalonFX {
    pub fn new(id: i32, bus: Option<String>) -> Self {
        let mut motor = com_ctre_phoenix6_hardware_TalonFX {
            inner: create_talon(id, bus),
        };

        Self {
            motor
        }
    }

    pub fn set(&self, value: f64) {
        com_ctre_phoenix6_hardware_TalonFX::set(&self.motor.inner(), value).unwrap()
    }

    pub fn stop(&self) {
        com_ctre_phoenix6_hardware_TalonFX::stopMotor(&self.motor.inner()).unwrap()
    }
}