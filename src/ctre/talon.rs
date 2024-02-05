use j4rs::{Instance, InvocationArg, Jvm};
use uom::si::f64::Angle;
use crate::ctre::TalonInvertType;
use crate::Motor;

use super::talon_encoder_tick;

pub struct Kraken {
    can_id: i32,
    pub(crate) instance: Instance
}

pub enum ControlMode {
    Percent,
    Velocity,
    Position
}

impl Kraken {
    pub fn new(can_id: i32, can_loop: Option<String>) -> Self {
        let jvm = Jvm::attach_thread().unwrap();

        /*let instance = jvm
            .create_instance(
                "com.ctre.phoenix.motorcontrol.can.WPI_TalonFX",
                &[InvocationArg::try_from(can_id)
                    .unwrap().into_primitive().unwrap(),
                    InvocationArg::try_from(can_loop.unwrap_or("rio".to_owned())).unwrap()
                ],
            ).unwrap();*/

        /*let instance = jvm.invoke_static("frc.robot.Wrapper", "createTalonFX", &[
            InvocationArg::try_from(can_id).unwrap().into_primitive().unwrap(),
            InvocationArg::try_from(can_loop.unwrap_or("rio".to_owned())).unwrap()
        ]).unwrap();*/

        let instance = jvm.create_instance(
            "com.ctre.phoenix6.hardware.TalonFX",
            &[InvocationArg::try_from(can_id)
                .unwrap().into_primitive().unwrap(),
                InvocationArg::try_from(can_loop.unwrap_or("rio".to_owned())).unwrap()
            ]).unwrap();

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

    pub fn follow(&self, master: Kraken) {
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

pub struct Falcon {
    can_id: i32,
    pub(crate) instance: Instance
}

impl Falcon {
    pub fn new(can_id: i32, can_loop: Option<String>) -> Self {
        let jvm = Jvm::attach_thread().unwrap();

        /*let instance = jvm
            .create_instance(
                "com.ctre.phoenix.motorcontrol.can.WPI_TalonFX",
                &[InvocationArg::try_from(can_id)
                    .unwrap().into_primitive().unwrap(),
                    InvocationArg::try_from(can_loop.unwrap_or("rio".to_owned())).unwrap()
                ],
            ).unwrap();*/

        /*let instance = jvm.invoke_static("frc.robot.Wrapper", "createTalonFX", &[
            InvocationArg::try_from(can_id).unwrap().into_primitive().unwrap(),
            InvocationArg::try_from(can_loop.unwrap_or("rio".to_owned())).unwrap()
        ]).unwrap();*/

        let instance = jvm.create_instance(
            "com.ctre.phoenix.motorcontrol.can.TalonFX",
            &[InvocationArg::try_from(can_id)
                .unwrap().into_primitive().unwrap(),
                InvocationArg::try_from(can_loop.unwrap_or("rio".to_owned())).unwrap()
            ]).unwrap();

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

    pub fn follow(&self, master: Kraken) {
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

    pub fn get(&self) -> Angle {
        let jvm = Jvm::attach_thread().unwrap();

        let returned: f64 = jvm.to_rust(jvm.invoke(
            &self.instance,
            "getSelectedSensorPosition",
            &Vec::new(),
        ).unwrap()).unwrap();

        let angle = Angle::new::<talon_encoder_tick>(returned);

        angle
    }
}
