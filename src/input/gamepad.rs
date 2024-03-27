use std::time::Instant;

use bitvec::prelude::*;
use jni::{objects::{GlobalRef, JObject, JValue}, signature::{Primitive, ReturnType}};
use once_cell::sync::OnceCell;

use crate::{call::{call, call_static, create, once}, java};

// https://github.com/wpilibsuite/allwpilib/blob/main/wpilibj/src/main/java/edu/wpi/first/wpilibj/XboxController.java
enum Buttons {
    LeftBumper = 5,
    RightBumper = 6,
    LeftStick = 9,
    RightStick = 10,
    A = 1,
    B = 2,
    X = 3,
    Y = 4,
    Back = 7,
    Start = 8,
}
enum Axis {
    LeftX = 0,
    RightX = 4,
    LeftY = 1,
    RightY = 5,
    LeftTrigger = 2,
    RightTrigger = 3,
}

pub struct Gamepad {
    id: i32,
    instance: GlobalRef,
    buttons: BitVec,
    last_updated: Instant,
}

impl Gamepad {
    pub fn new(id: i32) -> Self {
        let instance = create!(
            "edu/wpi/first/wpilibj/XboxController",
            "(I)V",
            &[JValue::Int(id).as_jni()]
        );

        let buttons = bitvec![0; 32];
        let last_updated = Instant::now();

        let instance = java().new_global_ref(instance).unwrap();

        Self { id, instance, buttons, last_updated }
    }

    pub fn left_y(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "edu/wpi/first/wpilibj/XboxController",
            "getLeftY",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn right_y(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "edu/wpi/first/wpilibj/XboxController",
            "getRightY",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn left_x(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "edu/wpi/first/wpilibj/XboxController",
            "getLeftX",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn right_x(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "edu/wpi/first/wpilibj/XboxController",
            "getRightX",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn left_trigger(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "edu/wpi/first/wpilibj/XboxController",
            "getLeftTriggerAxis",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn right_trigger(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "edu/wpi/first/wpilibj/XboxController",
            "getRightTriggerAxis",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    fn button(&mut self, id: usize) -> bool {
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

    pub fn left_bumper(&mut self) -> bool {
        self.button(Buttons::LeftBumper as usize)
    }

    pub fn right_bumper(&mut self) -> bool {
        self.button(Buttons::RightBumper as usize)
    }

    pub fn right_stick(&mut self) -> bool {
        self.button(Buttons::RightStick as usize)
    }

    pub fn a(&mut self) -> bool {
        self.button(Buttons::A as usize)
    }

    pub fn b(&mut self) -> bool {
        self.button(Buttons::B as usize)
    }

    pub fn x(&mut self) -> bool {
        self.button(Buttons::X as usize)
    }

    pub fn y(&mut self) -> bool {
        self.button(Buttons::Y as usize)
    }

    pub fn back(&mut self) -> bool {
        self.button(Buttons::Back as usize)
    }

    pub fn start(&mut self) -> bool {
        self.button(Buttons::Start as usize)
    }

    pub fn rumble_left(&mut self, strength: f64) {
        let left = once!(
            java().call_static_method(
                "frc/robot/Wrapper",
                "kLeftRumble",
                "()Ledu/wpi/first/wpilibj/GenericHID$RumbleType;",
                &Vec::new()
            ).unwrap().l().unwrap()
        );

        call!(
            self.instance.as_obj(),
            "com/revrobotics/CANSparkBase",
            "setIdleMode",
            "(Ledu/wpi/first/wpilibj/GenericHID$RumbleType;D)V",
            &[JValue::Object(&left).as_jni(),
            JValue::Double(strength).as_jni(),
            ],
            ReturnType::Primitive(Primitive::Void)
        );
    }

    pub fn rumble_right(&mut self, strength: f64) {
        let left = once!(
            java().call_static_method(
                "frc/robot/Wrapper",
                "kRightRumble",
                "()Ledu/wpi/first/wpilibj/GenericHID$RumbleType;",
                &Vec::new()
            ).unwrap().l().unwrap()
        );

        call!(
            self.instance.as_obj(),
            "com/revrobotics/CANSparkBase",
            "setIdleMode",
            "(Ledu/wpi/first/wpilibj/GenericHID$RumbleType;D)V",
            &[JValue::Object(&left).as_jni(),
            JValue::Double(strength).as_jni(),
            ],
            ReturnType::Primitive(Primitive::Void)
        );
    }
}
