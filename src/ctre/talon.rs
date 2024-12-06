use auto_jni::{call, create};
use auto_jni::jni::objects::{GlobalRef, JObject, JValue};
use auto_jni::once_cell;
use auto_jni::jni;
use auto_jni::jni::signature::{Primitive, ReturnType};
use crate::{com_ctre_phoenix6_controls_DutyCycleOut, com_ctre_phoenix6_hardware_core_CoreTalonFX, com_ctre_phoenix6_hardware_TalonFX, com_ctre_phoenix6_StatusSignal, java};

pub enum ControlMode {
    Percent,
    Position
}

pub struct TalonFX {
    motor: com_ctre_phoenix6_hardware_TalonFX,
}

impl TalonFX {
    pub fn new(id: i32, bus: Option<String>) -> Self {
        let string = java().new_string(bus.unwrap_or("rio".to_string())).unwrap();

        Self {
            motor: com_ctre_phoenix6_hardware_TalonFX::new_2(id, &JObject::from(string)).unwrap()
        }
    }

    pub fn set(&self, control_mode: ControlMode, value: f64) {
        match control_mode {
            ControlMode::Percent => {
                // Auto-jni doesn't properly create bindings to this class, so we have to do it manually
                let control = create!(
                    "com/ctre/phoenix6/controls/DutyCycleOut",
                    "(D)V",
                    &[JValue::Double(value).as_jni()]
                );

                com_ctre_phoenix6_hardware_core_CoreTalonFX::setControl(&self.motor.inner(), &control).unwrap();
            }
            ControlMode::Position => {
                let control = create!(
                    "com/ctre/phoenix6/controls/PositionDutyCycle",
                    "(D)V",
                    &[JValue::Double(value).as_jni()]
                );

                com_ctre_phoenix6_hardware_core_CoreTalonFX::setControl_4(&self.motor.inner(), &control).unwrap();
            }
        }
        com_ctre_phoenix6_hardware_TalonFX::set(&self.motor.inner(), value).unwrap()
    }

    pub fn stop(&self) {
        com_ctre_phoenix6_hardware_TalonFX::stopMotor(&self.motor.inner()).unwrap()
    }

    pub fn get_velocity(&self) -> f64 {
        let status = com_ctre_phoenix6_hardware_core_CoreTalonFX::getVelocity(&self.motor.inner()).unwrap();

        // Auto-jni has this function return an object, so we have to manually extract the value and have it return us a double
        call!(
            &status,
            "com/ctre/phoenix6/StatusSignal",
            "getValue",
            "()Ljava/lang/Object;",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn get_position(&self) -> f64 {
        let status = com_ctre_phoenix6_hardware_core_CoreTalonFX::getPosition(&self.motor.inner()).unwrap();

        call!(
            &status,
            "com/ctre/phoenix6/StatusSignal",
            "getValue",
            "()Ljava/lang/Object;",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }
}