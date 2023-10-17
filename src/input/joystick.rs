use j4rs::{Instance, InvocationArg, Jvm};

pub struct Joystick {
    id: i32,
    instance: Instance,
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

        Self { id, instance }
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

    pub fn get(&self, id: i32) -> bool {
        let jvm = Jvm::attach_thread().unwrap();

        let value: bool = jvm
            .to_rust(
                jvm.invoke(
                    &self.instance,
                    "getRawButton",
                    &[InvocationArg::try_from(id)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap(),
            )
            .unwrap();
        value
    }
}
