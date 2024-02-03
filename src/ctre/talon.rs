use j4rs::{Instance, InvocationArg, Jvm};
use crate::ctre::TalonInvertType;
use crate::Motor;

pub struct Talon {
    can_id: i32,
    pub(crate) instance: Instance
}

pub enum ControlMode {
    Percent,
    Velocity,
    Position
}

impl Talon {
    pub fn new(can_id: i32, can_loop: Option<String>) -> Self {
        let jvm = Jvm::attach_thread().unwrap();

        let instance = jvm
            .create_instance(
                "com.ctre.phoenix.motorcontrol.can.WPI_TalonFX",
                &[InvocationArg::try_from(can_id)
                    .unwrap().into_primitive().unwrap(),
                    InvocationArg::try_from(can_loop.unwrap_or("rio".to_owned())).unwrap()
                ],
            ).unwrap();

        Self { can_id, instance }
    }

    pub fn set(&self, control_mode: ControlMode, amount: f64) {
        let jvm = Jvm::attach_thread().unwrap();

        match control_mode {
            ControlMode::Percent => {
                jvm.invoke(
                    &self.instance,
                    "set",
                    &[InvocationArg::try_from(amount)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                ).unwrap();
            }
            ControlMode::Velocity => {
                let control = jvm.invoke_static(
                    "frc.robot.Wrapper",
                    "ctreVelocity",
                    &Vec::new()
                ).unwrap();

                jvm.invoke(
                    &self.instance,
                    "set",
                    &[
                        InvocationArg::from(control),
                        InvocationArg::try_from(amount)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                ).unwrap();
            }
            ControlMode::Position => {
                let control = jvm.invoke_static(
                    "frc.robot.Wrapper",
                    "ctrePosition",
                    &Vec::new()
                ).unwrap();

                jvm.invoke(
                    &self.instance,
                    "set",
                    &[
                        InvocationArg::from(control),
                        InvocationArg::try_from(amount)
                            .unwrap()
                            .into_primitive()
                            .unwrap()],
                ).unwrap();
            }
        }
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

    pub fn get(&self) -> f64 {
        let jvm = Jvm::attach_thread().unwrap();

        let returned: f64 = jvm.to_rust(jvm.invoke(
            &self.instance,
            "getSelectedSensorPosition",
            &Vec::new(),
        ).unwrap()).unwrap();

        returned
    }
}

/*impl Motor for Talon {
    fn set(&self, value: f64) {
        self.set(value);
    }

    fn stop(&self) {
        self.stop();
    }
}*/