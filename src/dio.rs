use j4rs::{Jvm, InvocationArg, Instance};
use jni::{objects::{JObject, JValue}, signature::{Primitive, ReturnType}};

use crate::call::*;

pub struct DIO<'local> {
    instance: JObject<'local>,
}

impl<'local> DIO<'local> {
    pub fn new(port: i32) -> Self { 
        let instance = create!(
            "edu/wpi/first/wpilibj/DigitalInput",
            "(I)V",
            &[JValue::Int(port).as_jni()]
        );

        Self { instance } 
    }

    pub fn get(&self) -> bool {
        call!(
            &self.instance,
            "edu/wpi/first/wpilibj/DigitalInput",
            "get",
            "()Z",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Boolean)
        ).z().unwrap()
    }
}
