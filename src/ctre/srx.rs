use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::Primitive::Void;
use jni::signature::ReturnType;
use crate::call::{call, call_static, create};

/// Represents a motor controller of type `TalonSRX`.
///
/// The `SRX` struct provides methods to instantiate a new `TalonSRX` motor controller
/// and set its output.
///
/// # Fields
///
/// - `instance`: An instance of the `TalonSRX` motor controller represented by a `GlobalRef`.
///
/// # Example
///
/// ```rust
/// use crate::ctre::srx::SRX;
///
/// let motor = SRX::new(1);  // Creates a new TalonSRX motor controller with ID 1
/// motor.set(0.5);           // Sets the motor output to 50%
/// ```
pub struct SRX {
    instance: GlobalRef
}

impl SRX {
    /// Creates a new instance of the `TalonSRX` motor controller with the given ID.
    ///
    /// This method initializes the Java object for `TalonSRX` and maintains a global reference to it.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the `TalonSRX` motor controller.
    ///
    /// # Returns
    ///
    /// A new instance of `SRX`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let motor = SRX::new(1);
    /// ```
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

    /// Sets the output of the `TalonSRX` motor controller.
    ///
    /// This method calls the `set` method on the `TalonSRX` motor controller with the specified
    /// control mode and output value.
    ///
    /// # Arguments
    ///
    /// * `value` - The output value to set on the motor controller (typically -1.0 to 1.0).
    ///
    /// # Example
    ///
    /// ```rust
    /// let motor = SRX::new(1);
    /// motor.set(0.5); // Sets the motor output to 50%
    /// ```
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