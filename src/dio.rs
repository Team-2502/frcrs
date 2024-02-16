use j4rs::{Jvm, InvocationArg, Instance};

pub struct DIO {
    instance: Instance,
}

impl DIO {
    pub fn new(port: i32) -> Self { 
        let jvm = Jvm::attach_thread().unwrap();

        let instance = jvm
            .create_instance(
                "edu.wpi.first.wpilibj.DigitalInput",
                &[
                    InvocationArg::try_from(port)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        Self { instance } 
    }

    pub fn get(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();

        jvm
            .to_rust(jvm.invoke(&self.instance, "get", &Vec::new()).unwrap())
            .unwrap()
    }
}
