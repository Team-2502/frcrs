use std::future::Future;
use std::time::Instant;

use crate::{
    call::{call, call_static, create},
    java, JAVA,
};
use bitvec::prelude::*;
use jni::{
    objects::{GlobalRef, JValue},
    signature::{Primitive, ReturnType},
};
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct Joystick {
    id: i32,
    instance: GlobalRef,
    buttons: BitVec,
    last_updated: Instant,
}

impl Joystick {
    /// Creates a new instance of `Joystick` with the given `id`.
    ///
    /// This method initializes a new Java object for the joystick and sets up its button states
    /// and last updated timestamp.
    ///
    /// # Arguments
    /// - `id`: An integer identifier for the joystick.
    ///
    /// # Returns
    /// A new instance of `Joystick`.
    pub fn new(id: i32) -> Self {
        JAVA.attach_current_thread_as_daemon().unwrap();
        let instance = create!(
            "edu/wpi/first/wpilibj/Joystick",
            "(I)V",
            &[JValue::Int(id).as_jni()]
        );

        let buttons = bitvec![0; 32];
        let last_updated = Instant::now();

        let instance = java().new_global_ref(instance).unwrap();

        Self {
            id,
            instance,
            buttons,
            last_updated,
        }
    }

    /// Gets the X-axis value of the joystick.
    ///
    /// This method calls the corresponding Java method to retrieve the X-axis value.
    ///
    /// # Returns
    /// The X-axis value as a `f64`.
    pub fn get_x(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "edu/wpi/first/wpilibj/Joystick",
            "getX",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap()
    }

    /// Gets the Y-axis value of the joystick.
    ///
    /// This method calls the corresponding Java method to retrieve the Y-axis value.
    ///
    /// # Returns
    /// The Y-axis value as a `f64`.
    pub fn get_y(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "edu/wpi/first/wpilibj/Joystick",
            "getY",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap()
    }

    /// Gets the Z-axis value of the joystick.
    ///
    /// This method calls the corresponding Java method to retrieve the Z-axis value.
    ///
    /// # Returns
    /// The Z-axis value as a `f64`.
    pub fn get_z(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "edu/wpi/first/wpilibj/Joystick",
            "getZ",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap()
    }

    /// Gets the throttle value of the joystick.
    ///
    /// This method calls the corresponding Java method to retrieve the throttle value.
    ///
    /// # Returns
    /// The throttle value as a `f64`.
    pub fn get_throttle(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "edu/wpi/first/wpilibj/Joystick",
            "getThrottle",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap()
    }

    /// Gets the state of the specified button by `id`.
    ///
    /// This method checks if the button states need to be refreshed based on the last updated timestamp,
    /// then retrieves the state of the specified button.
    ///
    /// # Arguments
    /// - `id`: The button identifier, starting from 1.
    ///
    /// # Returns
    /// `true` if the button is pressed, `false` otherwise.
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
        )
        .i()
        .unwrap();
        self.buttons[..].store(value);
        self.last_updated = Instant::now();
        self.buttons[id - 1]
    }

    pub fn get_pov(&self) -> i32 {
        call_static!(
            "edu/wpi/first/wpilibj/DriverStation",
            "getStickPOV",
            "(II)I",
            &[JValue::Int(self.id).as_jni(), JValue::Int(0).as_jni()],
            ReturnType::Primitive(Primitive::Int)
        )
        .i()
        .unwrap()
    }

    pub fn while_held<F, Fut>(&'static mut self, button_id: usize, action: F)
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        // Use a thread-local handle to manage the task
        thread_local! {
            static BUTTON_TASKS: std::cell::RefCell<std::collections::HashMap<usize, Option<JoinHandle<()>>>>
                = std::cell::RefCell::new(std::collections::HashMap::new());
        }

        BUTTON_TASKS.with(|tasks_cell| {
            let mut tasks = tasks_cell.borrow_mut();

            // Cancel any existing task for this button
            if let Some(Some(handle)) = tasks.get_mut(&button_id) {
                handle.abort();
            }

            // Spawn a new task that continuously checks the button
            let handle = tokio::task::spawn_local(async move {
                loop {
                    // Check if the button is still held
                    if !self.get(button_id) {
                        break;
                    }

                    // Run the provided action
                    action().await;

                    // Yield to prevent tight looping
                    tokio::task::yield_now().await;
                }
            });

            // Store the new task handle
            tasks.insert(button_id, Some(handle));
        });
    }
}
