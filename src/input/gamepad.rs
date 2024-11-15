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

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Other,
    None,
}

impl Direction {
    /// Converts an angle in degrees to the corresponding `Direction`.
    ///
    /// # Arguments
    ///
    /// - `degrees`: The angle in degrees.
    ///
    /// # Returns
    ///
    /// A `Direction` corresponding to the provided angle.
    ///
    /// # Examples
    ///
    /// ```
    /// let direction = Direction::from_degrees(90);
    /// assert_eq!(direction, Direction::Right);
    /// ```
    fn from_degrees(degrees: i32) -> Self {
        match degrees {
            0 => Self::Up,
            90 => Self::Right,
            180 => Self::Down,
            270 => Self::Left,
            -1 => Self::None,
            _  => Self::Other,

        }
    }
}

pub struct Gamepad {
    id: i32,
    instance: GlobalRef,
    buttons: BitVec,
    last_updated: Instant,
}

impl Gamepad {
    /// Constructs a new `Gamepad` instance with the specified `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier for the gamepad.
    ///
    /// This function initializes the gamepad instance, sets up an empty button state, and
    /// initializes the last updated timestamp.
    ///
    /// # Returns
    ///
    /// A new `Gamepad` instance.
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

    pub fn left_stick(&mut self) -> bool {
        self.button(Buttons::LeftStick as usize)
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
            "edu/wpi/first/wpilibj/GenericHID",
            "setRumble",
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
            "edu/wpi/first/wpilibj/GenericHID",
            "setRumble",
            "(Ledu/wpi/first/wpilibj/GenericHID$RumbleType;D)V",
            &[JValue::Object(&left).as_jni(),
            JValue::Double(strength).as_jni(),
            ],
            ReturnType::Primitive(Primitive::Void)
        );
    }

    pub fn get_dpad(&self) -> i32 {
        call!(
            &self.instance,
            "edu/wpi/first/wpilibj/GenericHID",
            "getPOV",
            "()I",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Int)
        ).i().unwrap()
    }

    pub fn get_dpad_direction(&self) -> Direction {
        Direction::from_degrees(
            call!(
                &self.instance,
                "edu/wpi/first/wpilibj/GenericHID",
                "getPOV",
                "()I",
                &Vec::new(),
                ReturnType::Primitive(Primitive::Int)
            ).i().unwrap()
        )
    }
}
