use std::collections::HashMap;

use j4rs::{Instance, InvocationArg, Jvm};

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

        let instance = jvm.create_instance(
            "edu.wpi.first.wpilibj.smartdashboard.SendableChooser",
            &[ ]
        ).unwrap();

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

    pub fn get(&self) -> &T {
        let jvm = Jvm::attach_thread().unwrap();
        let idx: i32 = jvm.to_rust(jvm.invoke(
            &self.instance,
            "getSelected",
            &Vec::new(),
        ).unwrap()).unwrap();

        &self.options[idx as usize]
    }
}
