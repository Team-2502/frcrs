use std::time::Instant;

use bitvec::prelude::*;
use jni::{objects::{JObject, JValue}, signature::{Primitive, ReturnType}};

use crate::call::{call, call_static, create};


pub struct Joystick<'local> {
    id: i32,
    instance: JObject<'local>,
    buttons: BitVec,
    last_updated: Instant,
}

impl<'local> Joystick<'local> {
    pub fn new(id: i32) -> Self {
        let instance = create!(
            "edu/wpi/first/wpilibj/Joystick",
            "(I)V",
            &[JValue::Int(id).as_jni()]
        );

        let buttons = bitvec![0; 32];
        let last_updated = Instant::now();

        Self { id, instance, buttons, last_updated }
    }

    pub fn get_x(&self) -> f64 {
        call!(
            &self.instance,
            "edu/wpi/first/wpilibj/Joystick",
            "getX",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn get_y(&self) -> f64 {
        call!(
            &self.instance,
            "edu/wpi/first/wpilibj/Joystick",
            "getY",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn get_z(&self) -> f64 {
        call!(
            &self.instance,
            "edu/wpi/first/wpilibj/Joystick",
            "getZ",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn get_throttle(&self) -> f64 {
        call!(
            &self.instance,
            "edu/wpi/first/wpilibj/Joystick",
            "getThrottle",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn get(&mut self, id: usize) -> bool {
        if self.last_updated.elapsed().as_millis() < 15 {
            return self.buttons[id - 1];
        }

        let value = call_static!(
            "edu/wpi/first/wpilibj/DriverStation",
            "getStickButtons",
            "(I)I",
            &[JValue::Int(self.id).as_jni()],
            ReturnType::Primitive(Primitive::Int)
        ).i().unwrap();
        self.buttons[..].store(value);
        self.last_updated = Instant::now();
        self.buttons[id-1]
    }
}
