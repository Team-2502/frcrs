use crate::call::{call, call_static, create};
use crate::java;
use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::{Primitive, ReturnType};
use jni::sys::jboolean;

pub enum ControlMode {
    Percent,
    Position,
    MotionMagic,
}

/// Represents a motor controller of type `TalonFX`.
///
/// The `Talon` struct provides methods to instantiate a new `TalonFX` motor controller
/// and set its output.
///
/// # Example
///
/// ```rust
/// use frcrs::ctre::ControlMode;
/// use crate::ctre::talon::TalonFX;
///
/// let motor = Talon::new(1, None);  // Creates a new TalonSRX motor controller with ID 1
/// motor.set(ControlMode::Percent, 0.5);           // Sets the motor output to 50%
/// ```
pub struct Talon {
    pub(crate) instance: GlobalRef,
    pub(crate) id: i32,
}

impl Talon {
    /// Creates a new Talon motor controller instance.
    ///
    /// # Arguments
    /// - `id`: The numerical identifier for the motor controller.
    /// - `can_bus`: An optional string specifying the CAN bus on which the motor controller is connected. Defaults to "rio" if not provided.
    ///
    /// # Returns
    /// A new instance of `Talon`.
    ///
    /// # Example
    /// ```rust
    /// let talon = Talon::new(1, Some("can0".to_string()));
    /// ```
    pub fn new(id: i32, can_bus: Option<String>) -> Self {
        let string = java()
            .new_string(can_bus.unwrap_or("rio".to_string()))
            .unwrap();

        let instance = create!(
            "com/ctre/phoenix6/hardware/TalonFX",
            "(ILjava/lang/String;)V",
            &[
                JValue::Int(id).as_jni(),
                JValue::Object(&JObject::from(string)).as_jni()
            ]
        );

        let instance = java().new_global_ref(instance).unwrap();

        Self { instance, id }
    }

    /// Sets the output for the motor controller.
    ///
    /// # Arguments
    /// - `control_mode`: The control mode for the motor (e.g., `ControlMode::Percent` or `ControlMode::Position`).
    /// - `amount`: The desired output amount (e.g., a percentage for `ControlMode::Percent` or a position value for `ControlMode::Position`).
    ///
    /// # Example
    /// ```rust
    /// talon.set(ControlMode::Percent, 0.5);
    /// ```
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
                )
                .l()
                .unwrap();
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
            ControlMode::MotionMagic => {
                let control = create!(
                    "com/ctre/phoenix6/controls/MotionMagicDutyCycle",
                    "(D)V",
                    &[JValue::Double(amount).as_jni()]
                );

                call!(
                    self.instance.as_obj(),
                    "com/ctre/phoenix6/hardware/core/CoreTalonFX",
                    "setControl",
                    "(Lcom/ctre/phoenix6/controls/MotionMagicDutyCycle;)Lcom/ctre/phoenix6/StatusCode;",
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
        )
        .v()
        .unwrap()
    }

    /// Retrieves the current velocity of the motor.
    ///
    /// # Returns
    /// A `f64` value representing the motor's current velocity.
    ///
    /// # Example
    /// ```rust
    /// let velocity = talon.get_velocity();
    /// ```
    pub fn get_velocity(&self) -> f64 {
        let status_signal = call!(
            self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/core/CoreTalonFX",
            "getVelocity",
            "()Lcom/ctre/phoenix6/StatusSignal;",
            &Vec::new(),
            ReturnType::Object
        )
        .l()
        .unwrap();

        call_static!(
            "frc/robot/Wrapper",
            "getValue",
            "(Lcom/ctre/phoenix6/StatusSignal;)D",
            &[JValue::Object(&status_signal).as_jni()],
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap()
    }

    /// Retrieves the current position of the motor.
    ///
    /// # Returns
    /// A `f64` value representing the motor's current position.
    ///
    /// # Example
    /// ```rust
    /// let position = talon.get_position();
    /// ```
    pub fn get_position(&self) -> f64 {
        let status_signal = call!(
            self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/core/CoreTalonFX",
            "getPosition",
            "()Lcom/ctre/phoenix6/StatusSignal;",
            &Vec::new(),
            ReturnType::Object
        )
        .l()
        .unwrap();

        call_static!(
            "frc/robot/Wrapper",
            "getValue",
            "(Lcom/ctre/phoenix6/StatusSignal;)D",
            &[JValue::Object(&status_signal).as_jni()],
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap()
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn follow(&self, master: &Talon, inverted: bool) {
        let alignment = if inverted {
            call_static!(
                "frc/robot/Wrapper",
                "invertFollow",
                "()Lcom/ctre/phoenix6/signals/MotorAlignmentValue;",
                &Vec::new(),
                ReturnType::Object
            )
            .l()
            .unwrap()
        } else {
            call_static!(
                "frc/robot/Wrapper",
                "allignedFollow",
                "()Lcom/ctre/phoenix6/signals/MotorAlignmentValue;",
                &Vec::new(),
                ReturnType::Object
            )
            .l()
            .unwrap()
        };

        let follower = create!(
            "com/ctre/phoenix6/controls/Follower",
            "(ILcom/ctre/phoenix6/signals/MotorAlignmentValue;)V",
            &[
                JValue::Int(master.get_id()).as_jni(),
                JValue::Object(&alignment).as_jni(),
            ]
        );

        call!(
            self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/core/CoreTalonFX",
            "setControl",
            "(Lcom/ctre/phoenix6/controls/Follower;)Lcom/ctre/phoenix6/StatusCode;",
            &[JValue::Object(&follower).as_jni()],
            ReturnType::Object
        )
        .l()
        .unwrap();
    }

    pub fn zero(&self) {
        call!(
            &self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/core/CoreTalonFX",
            "setPosition",
            "(D)Lcom/ctre/phoenix6/StatusCode;",
            &[JValue::Double(0.).as_jni()],
            ReturnType::Object
        )
        .l()
        .unwrap();
    }

    /// Retrieves the stator current from the motor in Amps.
    ///
    /// # Returns
    /// A `f64` value representing the current drawn by the motor stator.
    ///
    /// # Example
    /// ```rust
    /// let current = talon.get_current();
    /// ```
    pub fn get_current(&self) -> f64 {
        let status_signal = call!(
            self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/core/CoreTalonFX",
            "getStatorCurrent",
            "()Lcom/ctre/phoenix6/StatusSignal;",
            &Vec::new(),
            ReturnType::Object
        )
        .l()
        .unwrap();

        call_static!(
            "frc/robot/Wrapper",
            "getValue",
            "(Lcom/ctre/phoenix6/StatusSignal;)D",
            &[JValue::Object(&status_signal).as_jni()],
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap()
    }
}
