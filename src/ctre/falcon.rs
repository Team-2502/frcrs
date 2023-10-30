use j4rs::Instance;
use j4rs::InvocationArg;
use j4rs::Jvm;

pub struct Falcon {
    can_id: i32,
    instance: Instance,
}

impl Falcon {
    pub fn new(can_id: i32, can_loop: Option<String>) -> Self {
        let jvm = Jvm::attach_thread().unwrap();

        let instance = jvm
            .create_instance(
                "com.ctre.phoenix.motorcontrol.can",
                &[InvocationArg::try_from(can_id)
                    .unwrap().into_primitive().unwrap(),
                InvocationArg::try_from(can_loop.unwrap_or(String::new()))
                    .unwrap().into_primitive().unwrap()],
            )
            .unwrap();

        Self { can_id, instance }
    }

    pub fn set(&self, speed: f64) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke(
            &self.instance,
            "set",
            &[InvocationArg::try_from(speed)
                .unwrap()
                .into_primitive()
                .unwrap()],
        )
        .unwrap();
    }

    pub fn stop(&self) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke(&self.instance, "stopMotor", &Vec::new())
            .unwrap();
    }
}
