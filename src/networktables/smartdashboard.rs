use std::collections::HashMap;

use j4rs::{Instance, InvocationArg, Jvm};
use uom::si::{f64::Angle, angle::radian};
use nalgebra::Vector2;

pub struct SmartDashboard;

impl SmartDashboard {
    pub fn init() {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke_static(
            "frc.robot.Wrapper",
            "startNetworkTables",
            &[]
        )
            .unwrap();
    }


    pub fn put_number(key: String, data: f64) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke_static(
            "edu.wpi.first.wpilibj.smartdashboard.SmartDashboard",
            "putNumber",
            &[
                InvocationArg::try_from(key).unwrap(),
                InvocationArg::try_from(data).unwrap().into_primitive().unwrap(),
            ]
        )
        .unwrap();
    }

    pub fn put_bool(key: String, data: bool) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke_static(
            "edu.wpi.first.wpilibj.smartdashboard.SmartDashboard",
            "putBoolean",
            &[
                InvocationArg::try_from(key).unwrap(),
                InvocationArg::try_from(data).unwrap().into_primitive().unwrap(),
            ]
        )
        .unwrap();
    }
}

pub struct Chooser<T> {
    options: Vec<T>,
    instance: Instance,
}

impl<T> Chooser<T> {
    pub fn new() -> Self {
        let options = Vec::new();
        let jvm = Jvm::attach_thread().unwrap();

            let instance = jvm.invoke_static(
                "frc.robot.Wrapper",
                "autoChooser",
                &Vec::new()).unwrap();

        Self { options, instance }
    }

    pub fn add(&mut self, name: &str, option: T) {
        self.options.push(option);
        let idx = self.options.len();
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke(
            &self.instance,
            "addOption",
            &[
                InvocationArg::try_from(name.to_owned()).unwrap(),
                InvocationArg::try_from(idx as i32).unwrap().into_primitive().unwrap(),
            ],
        ).unwrap();
    }

    pub fn get(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let idx: i32 = jvm.to_rust(jvm.invoke(
            &self.instance,
            "getSelected",
            &Vec::new(),
        ).unwrap()).unwrap();

        idx
    }

}

pub fn set_position(position: Vector2<f64>, angle: Angle) {
    let jvm = Jvm::attach_thread().unwrap();

    let angle = angle.get::<radian>();

    jvm.invoke_static(
        "frc.robot.Wrapper",
        "setPosition",
        &[
            InvocationArg::try_from(position.x).unwrap().into_primitive().unwrap(),
            InvocationArg::try_from(position.y).unwrap().into_primitive().unwrap(),
            InvocationArg::try_from(angle).unwrap().into_primitive().unwrap(),
        ]
    )
    .unwrap();
}
