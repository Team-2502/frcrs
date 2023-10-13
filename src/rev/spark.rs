use j4rs::{Instance, InvocationArg, Jvm};
use crate::rev::MotorType;

pub struct Spark {
    can_id: i32,
    motor_type: MotorType,
    instance: Instance
}

impl Spark {
    pub fn new(can_id: i32, motor_type: MotorType) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "com.revrobotics.CANSparkMax",
                &[
                    InvocationArg::try_from(can_id)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from({
                        let motor_types = jvm
                            .static_class("com.revrobotics.CANSparkMaxLowLevel.MotorType")
                            .unwrap();
                        let k_brushless = jvm.field(&motor_types, motor_type.as_str()).unwrap();
                        k_brushless
                    })
                        .unwrap(),
                ],
            )
            .unwrap();

        Self {
            can_id,
            motor_type,
            instance
        }
    }

    /// Set the speed of the motor
    ///
    /// `amount` is from -1, 1
    pub fn set(&self, amount: f32) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(
            &self.instance,
            "set",
            &[InvocationArg::try_from(amount)
                .unwrap()
                .into_primitive()
                .unwrap()],
        ).expect("Failed to set motor}");
    }

    /// Stop the motor
    pub fn stop(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(&self.instance, "stopMotor", &Vec::new()).unwrap();
    }
}