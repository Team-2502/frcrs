use j4rs::{Instance, InvocationArg, Jvm};
use crate::ctre::TalonInvertType;

pub struct Talon {
    can_id: i32,
    pub(crate) instance: Instance
}

impl Talon {
    pub fn new(can_id: i32, can_loop: Option<String>) -> Self {
        let jvm = Jvm::attach_thread().unwrap();

        let instance = jvm
            .create_instance(
                "com.ctre.phoenix.motorcontrol.can.WPI_TalonFX",
                &[InvocationArg::try_from(can_id)
                    .unwrap().into_primitive().unwrap(),
                ],
            ).unwrap();

        Self { can_id, instance }
    }

    pub fn set(&self, amount: f64) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke(
            &self.instance,
            "set",
            &[InvocationArg::try_from(amount)
                .unwrap()
                .into_primitive()
                .unwrap()],
        ).unwrap();
    }

    pub fn follow(&self, master: Talon) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke(
            &self.instance,
            "follow",
            &[InvocationArg::try_from(master.instance)
                .unwrap()
                .into_primitive()
                .unwrap()],
        ).unwrap();
    }

    pub fn set_inverted(&self, talon_invert_type: TalonInvertType) {
        let jvm = Jvm::attach_thread().unwrap();

        let invert_type = jvm
            .invoke_static("frc.robot.Wrapper", talon_invert_type.as_str(), &Vec::new())
            .unwrap();

        jvm.invoke(
            &self.instance,
            "setInverted",
            &[
                InvocationArg::try_from(invert_type).unwrap(),
            ],
        )
            .unwrap();
    }

    pub fn stop(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(
            &self.instance,
            "stopMotor",
            &Vec::new(),
        ).unwrap();
    }
}