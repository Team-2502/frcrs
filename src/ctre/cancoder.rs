use j4rs::{Instance, InvocationArg, Jvm};

pub struct CanCoder {
    instance: Instance
}

impl CanCoder {
    pub fn new(id: i32, can_loop: Option<String>) -> Self {
        let jvm = Jvm::attach_thread().unwrap();

        let instance = jvm.create_instance(
            "com.ctre.phoenix.sensors.CANCoder",
            &[
                InvocationArg::try_from(id).unwrap().into_primitive().unwrap(),
                InvocationArg::try_from(can_loop.unwrap_or("rio".to_owned())).unwrap()
            ]
        ).unwrap();

        Self {
            instance
        }
    }

    pub fn get(&self) -> f64 {
        let jvm = Jvm::attach_thread().unwrap();

        let result: f64 = jvm.to_rust(jvm.invoke(
            &self.instance,
            "getPosition",
            &Vec::new(),
        ).unwrap()).unwrap();

        result
    }

    pub fn get_absolute(&self) -> f64 {
        let jvm = Jvm::attach_thread().unwrap();

        let result: f64 = jvm.to_rust(jvm.invoke(
            &self.instance,
            "getAbsolutePosition",
            &Vec::new(),
        ).unwrap()).unwrap();

        result
    }
}