use j4rs::{Instance, InvocationArg, Jvm};
use crate::rev::Spark;

pub struct SparkPIDController<'a> {
    motor: &'a Instance,
    controller: Instance,
    p: f64,
    i: f64,
    d:  f64
}

impl<'a> SparkPIDController<'a> {
    pub fn new(motor: &'a Spark) -> Self {
        let jvm = Jvm::attach_thread().unwrap();

        Self {
            motor: motor.instance(),
            controller: jvm.invoke(&motor.instance(), "getPIDController", &Vec::new()).unwrap(),
            p: 0.0,
            i: 0.0,
            d: 0.0
        }
    }

    pub(crate) fn from(motor: &'a Instance, controller: Instance, p: f64, i: f64, d: f64) -> Self {
        Self {
            motor,
            controller,
            p,
            i,
            d
        }
    }

    pub(crate) fn instance(&self) -> &Instance {
        &self.controller
    }

    pub fn set_p(&self, p: f64) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke(&self.controller, "setP", &[InvocationArg::try_from(p).unwrap().into_primitive().unwrap()]).unwrap();
    }

    pub fn set_i(&self, i: f64) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke(&self.controller, "setI", &[InvocationArg::try_from(i).unwrap().into_primitive().unwrap()]).unwrap();
    }

    pub fn set_d(&self, d: f64) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke(&self.controller, "setD", &[InvocationArg::try_from(d).unwrap().into_primitive().unwrap()]).unwrap();
    }

    pub fn set_i_zone(&self, iz: f64) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke(&self.controller, "setIZone", &[InvocationArg::try_from(iz).unwrap().into_primitive().unwrap()]).unwrap();
    }

    pub fn set_ff(&self, ff: f64) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke(&self.controller, "setFF", &[InvocationArg::try_from(ff).unwrap().into_primitive().unwrap()]).unwrap();
    }

    pub fn set_output_range(&self, min: f64, max: f64) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke(&self.controller, "setOutputRange", &[
            InvocationArg::try_from(min).unwrap().into_primitive().unwrap(),
            InvocationArg::try_from(max).unwrap().into_primitive().unwrap()
        ]).unwrap();
    }

    pub fn get_p(&self) -> f64 {
        self.p
    }

    pub fn get_i(&self) -> f64 {
        self.i
    }

    pub fn get_d(&self) -> f64 {
        self.d
    }
}
