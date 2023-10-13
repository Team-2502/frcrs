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

        let motortype = jvm.invoke_static("frc.robot.Wrapper", motor_type.as_str(), &Vec::new()).unwrap();

        let instance = jvm.create_instance("com.revrobotics.CANSparkMax", &[
            InvocationArg::try_from(can_id).unwrap().into_primitive().unwrap(),
            InvocationArg::try_from(motortype).unwrap()
        ]).unwrap();

        Self {
            can_id,
            motor_type,
            instance
        }
    }

    /// Set the speed of the motor
    ///
    /// `amount` is from -1, 1
    pub fn set(&self, amount: f64) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(&self.instance, "set", &[
            InvocationArg::try_from(amount).unwrap().into_primitive().unwrap(),
        ]).unwrap();//.expect("Failed to call `set` on motor");
    }

    /// Stop the motor
    pub fn stop(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(&self.instance, "stopMotor", &Vec::new()).unwrap();
    }
}
