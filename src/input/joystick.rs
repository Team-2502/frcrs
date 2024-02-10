use std::time::Instant;

use j4rs::{Instance, InvocationArg, Jvm};

use bitvec::prelude::*;


pub struct Joystick {
    id: i32,
    instance: Instance,
    buttons: BitVec,
    last_updated: Instant,
}

impl Joystick {
    pub fn new(id: i32) -> Self {
        let jvm = Jvm::attach_thread().unwrap();

        let instance = jvm
            .create_instance(
                "edu.wpi.first.wpilibj.Joystick",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();

        let buttons = bitvec![0; 32];
        let last_updated = Instant::now();

        Self { id, instance, buttons, last_updated }
    }

    pub fn get_x(&self) -> f64 {
        let jvm = Jvm::attach_thread().unwrap();

        let value: f64 = jvm
            .to_rust(jvm.invoke(&self.instance, "getX", &Vec::new()).unwrap())
            .unwrap();
        value
    }

    pub fn get_y(&self) -> f64 {
        let jvm = Jvm::attach_thread().unwrap();

        let value: f64 = jvm
            .to_rust(jvm.invoke(&self.instance, "getY", &Vec::new()).unwrap())
            .unwrap();
        value
    }

    pub fn get_z(&self) -> f64 {
        let jvm = Jvm::attach_thread().unwrap();

        let value: f64 = jvm
            .to_rust(jvm.invoke(&self.instance, "getZ", &Vec::new()).unwrap())
            .unwrap();
        value
    }

    pub fn get_throttle(&self) -> f64 {
        let jvm = Jvm::attach_thread().unwrap();

        let value: f64 = jvm
            .to_rust(jvm.invoke(&self.instance, "getThrottle", &Vec::new()).unwrap())
            .unwrap();
        -value
    }

    pub fn get(&mut self, id: usize) -> bool {

        if self.last_updated.elapsed().as_millis() < 15 {
            return self.buttons[id - 1];
        }

        let jvm = Jvm::attach_thread().unwrap();

        let value: i32 = jvm
            .to_rust(
                jvm.invoke_static(
                    "edu.wpi.first.wpilibj.DriverStation",
                    "getStickButtons",
                    &[
                    InvocationArg::try_from(self.id)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    ],
                )
                .unwrap(),
            )
            .unwrap();
        self.buttons[..].store(value);
        self.last_updated = Instant::now();
        self.buttons[id-1]
    }
}
