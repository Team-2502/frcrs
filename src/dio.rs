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

pub struct DO {
    instance: Instance,
}

impl DO {
    pub fn new(port: i32) -> Self { 
        let jvm = Jvm::attach_thread().unwrap();

        let instance = jvm
            .create_instance(
                "edu.wpi.first.wpilibj.DigitalOutput",
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

    pub fn set(&self, value: bool) {
        let jvm = Jvm::attach_thread().unwrap();
        println!("crash");

        jvm.invoke(&self.instance, "set", &[
            InvocationArg::try_from(value)
            .unwrap().into_primitive().unwrap()
        ]).unwrap();
    }
}
