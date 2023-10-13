use j4rs::{Instance, InvocationArg, Jvm};
use crate::rev::{ControlType, IdleMode, MotorType, SparkPIDController};

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

    pub fn set_idle_mode(&self, idle_mode: IdleMode) {
        let jvm = Jvm::attach_thread().unwrap();

        let mode = jvm.invoke_static("frc.robot.Wrapper", idle_mode.as_str(), &Vec::new()).unwrap();

        jvm.invoke(&self.instance, "setIdleMode", &[
            InvocationArg::try_from(mode).unwrap(),
        ]).unwrap();
    }

    pub(crate) fn instance(&self) -> &Instance {
        &self.instance
    }

    pub fn get_pid(&self) -> SparkPIDController {
        let jvm = Jvm::attach_thread().unwrap();

        // TODO: get values from spark
        SparkPIDController::from(&self.instance, jvm.invoke(&self.instance, "getPIDController", &Vec::new()).unwrap(), 0.0, 0.0, 0.0)
    }

    pub fn set_reference(&self, value: f64, control_type: ControlType) {
        let jvm = Jvm::attach_thread().unwrap();

        let _control_type = jvm.invoke_static("frc.robot.Wrapper", control_type.as_str(), &Vec::new()).unwrap();

        jvm.invoke(&self.get_pid().instance(), "setReference", &[InvocationArg::try_from(value).unwrap().into_primitive().unwrap(), InvocationArg::try_from(_control_type).unwrap()]).unwrap();
    }

    /// Stop the motor
    pub fn stop(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(&self.instance, "stopMotor", &Vec::new()).unwrap();
    }
}

