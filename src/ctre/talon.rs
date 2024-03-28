use jni::objects::{GlobalRef, JClass, JMethodID, JObject, JValue};
use jni::signature::{Primitive, ReturnType};
use once_cell::sync::OnceCell;
use crate::call::{call, create};
use crate::java;

pub enum ControlMode {
    Percent,
    Position
}

pub struct Talon {
    pub(crate) instance: GlobalRef
}

impl Talon {
    /// Constructs a new Talon FX motor controller object.
    /// # Parameters:
    /// - id - ID of the device, as configured in Phoenix Tuner.
    /// - can_bus - Name of the CAN bus this device is on,
    /// None will select rio
    pub fn new(id: i32, can_bus: Option<String>) -> Self {
        let string = java().new_string(can_bus.unwrap_or("rio".to_string())).unwrap();

        let instance = create!(
            "com/ctre/phoenix6/hardware/TalonFX",
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

    /// Request selected output for motor controller
    /// # Paramaters
    /// - control_mode: the requested control mode [ControlMode]
    /// - amount: the amount requested for the control
    pub fn set(&self, control_mode: ControlMode, amount: f64) {
        match control_mode {
            ControlMode::Percent => {
                let control = create!(
                    "com/ctre/phoenix6/controls/DutyCycleOut",
                    "(D)V",
                    &[JValue::Double(amount).as_jni()]
                );

                call!(
                    self.instance.as_obj(),
                    "com/ctre/phoenix6/hardware/core/CoreTalonFX",
                    "setControl",
                    "(Lcom/ctre/phoenix6/controls/DutyCycleOut;)Lcom/ctre/phoenix6/StatusCode;",
                    &[JValue::Object(&control.as_obj()).as_jni()],
                    ReturnType::Object
                ).l().unwrap();
            }
            ControlMode::Position => {
                let control = create!(
                    "com/ctre/phoenix6/controls/PositionDutyCycle",
                    "(D)V",
                    &[JValue::Double(amount).as_jni()]
                );

                call!(
                    self.instance.as_obj(),
                    "com/ctre/phoenix6/hardware/core/CoreTalonFX",
                    "setControl",
                    "(Lcom/ctre/phoenix6/controls/PositionDutyCycle;)Lcom/ctre/phoenix6/StatusCode;",
                    &[JValue::Object(&control.as_obj()).as_jni()],
                    ReturnType::Object
                ).l().unwrap();
            }
        };
    }

    /// Stop the motor
    pub fn stop(&self) {
        call!(
            self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/TalonFX",
            "stopMotor",
            "()V",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Void)
        ).v().unwrap()
    }

    /// Get the current velocity of the motor
    pub fn get_velocity(&self) -> f64 {
        let status_signal = call!(
            self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/core/CoreTalonFX",
            "getVelocity",
            "()Lcom/ctre/phoenix6/StatusSignal;",
            &Vec::new(),
            ReturnType::Object
        ).l().unwrap();

        call!(
            &status_signal,
            "com/ctre/phoenix6/StatusSignal",
            "getValue",
            "()Ljava/lang/Object;",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    /// Get the current position of the motor
    pub fn get_position(&self) -> f64 {
        let status_signal = call!(
            self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/core/CoreTalonFX",
            "getPosition",
            "()Lcom/ctre/phoenix6/StatusSignal;",
            &Vec::new(),
            ReturnType::Object
        ).l().unwrap();

        call!(
            &status_signal,
            "com/ctre/phoenix6/StatusSignal",
            "getValue",
            "()Ljava/lang/Object;",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }
}
