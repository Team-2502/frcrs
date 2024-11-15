use jni::{objects::{GlobalRef, JObject, JValue}, signature::{Primitive, ReturnType}};

use crate::call::*;

pub struct DIO {
    instance: GlobalRef,
}

impl DIO {
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
