use j4rs::{Instance, Jvm};

pub struct NavX {
    instance: Instance,
}

impl NavX {
    pub fn new() -> Self {
        let jvm = Jvm::attach_thread().unwrap();

        /*let instance = jvm.create_instance(
            "com.kauailabs.navx.frc.AHRS",
            &Vec::new(),
        ).unwrap();*/

        let instance = jvm.invoke_static(
            "frc.robot.Wrapper",
            "createAHRS",
            &Vec::new(),
        ).unwrap();

        Self {
            instance
        }
    }

    pub fn get_angle(&self) -> f64 {
        let jvm = Jvm::attach_thread().unwrap();

        let angle: f64 = jvm.to_rust(
        jvm.invoke(
             &self.instance,
            "getAngle",
            &Vec::new(),
        ).unwrap()).unwrap();

        angle
    }

    pub fn reset_angle(&self) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke(
            &self.instance,
            "reset",
            &Vec::new(),
        ).unwrap();
    }
}