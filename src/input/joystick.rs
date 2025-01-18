use std::future::Future;
use std::time::Instant;

use bitvec::prelude::*;
use jni::{objects::{GlobalRef, JValue}, signature::{Primitive, ReturnType}};
use tokio::task::JoinHandle;
use crate::{call::{call, call_static, create}, java, JAVA};


pub struct Joystick {
    id: i32,
    instance: GlobalRef,
    buttons: BitVec,
    last_updated: Instant,
}

impl Joystick {
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

        Self { id, instance, buttons, last_updated }
    }

    pub fn get_x(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "edu/wpi/first/wpilibj/Joystick",
            "getX",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn get_y(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "edu/wpi/first/wpilibj/Joystick",
            "getY",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn get_z(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "edu/wpi/first/wpilibj/Joystick",
            "getZ",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn get_throttle(&self) -> f64 {
        call!(
            self.instance.as_obj(),
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
