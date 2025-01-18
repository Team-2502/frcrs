pub mod ctre;
pub mod input;
pub mod networktables;
pub mod rev;
pub mod navx;
pub mod drive;
pub mod dio;
#[macro_use]
pub mod call;
pub mod led;
pub mod solenoid;
pub mod telemetry;
pub mod limelight;

use std::any::TypeId;
use std::cmp::PartialEq;
use std::future::Future;
use std::hash::{Hash, Hasher};
pub use j4rs_derive::call_from_java;
use jni::objects::{JObject, JValue};
use jni::signature::Primitive;
use jni::{InitArgsBuilder, JNIEnv, JNIVersion, JavaVM};
use lazy_static::lazy_static;
use tokio::task::{spawn_local, AbortHandle, LocalSet};
use tokio::time::{interval, sleep};
use crate::input::{RobotMode, RobotState};

#[macro_use]
extern crate uom;

use std::convert::TryFrom;
use std::ops::Range;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use crate::drive::ToTalonEncoder;
use std::collections::HashMap;
use std::sync::Arc;

fn create_jvm() -> JavaVM{
    // set JAVA_HOME to /usr/local/frc/JRE/bin/
    let jvm_args = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        .option("-XX:+UseSerialGC")
        .option("-Djava.lang.invoke.stringConcat=BC_SB")
        .option("-Djava.library.path=/usr/local/frc/third-party/lib")
        .option("-Djava.class.path=/home/lvuser/javastub.jar")
        .build().unwrap();

    let jvm = JavaVM::with_libjvm(jvm_args, || Ok("/usr/local/frc/JRE/lib/client/libjvm.so")).unwrap();
    jvm.attach_current_thread_as_daemon().unwrap();
    jvm
}

lazy_static!{
    static ref  JAVA: JavaVM = create_jvm();
}

fn java() -> JNIEnv<'static> {
    JAVA.attach_current_thread_permanently().unwrap()
}

/// Map x (within from) to the same relative spot in to
pub fn deadzone(input: f64, from_range: &Range<f64>, to_range: &Range<f64>) -> f64 {
    let neg = input < 0.0;
    let input = input.abs();
    let from_len = from_range.end - from_range.start;
    let to_len = to_range.end - to_range.start;
    let standard = (input - from_range.start) / from_len;
    let mut out = (standard * to_len) + to_range.start;
    if input < from_range.start { out = 0.0 };
    if neg {
        -out
    } else {
        out
    }
}
/*
#[cfg(test)]
mod tests {
    use super::deadzone;
    #[test]
    fn deadzone_test() {
        let result = deadzone(0.5, &(0.0..1.0), &(0.0..2.0));
        assert_eq!(result, 1.0);
    }

    #[test]
    fn deadzone_reverse_test() {
        let result = deadzone(-0.5, &(0.0..1.0), &(0.0..2.0));
        assert_eq!(result, -1.0);
    }

    #[test]
    fn deadzone_reverse_test_2() {
        let result = deadzone(-0.75, &(0.5..1.0), &(0.0..2.0));
        assert_eq!(result, -1.0);
    }

    #[test]
    fn deadzone_test_2() {
        let result = deadzone(0.75, &(0.5..1.0), &(0.0..2.0));
        assert_eq!(result, 1.0);
    }
}*/

pub fn observe_user_program_starting() {
    // Show "robot code" on driver's station
    call_static!(
        "edu/wpi/first/hal/DriverStationJNI",
        "observeUserProgramStarting",
        "()V",
        &Vec::new(),
        jni::signature::ReturnType::Primitive(Primitive::Void)
    );
}

pub fn refresh_data() {
    call_static!(
        "edu/wpi/first/wpilibj/DriverStation",
        "refreshData",
        "()V",
        &Vec::new(),
        jni::signature::ReturnType::Primitive(Primitive::Void)
    );
}

pub fn init_hal() -> bool {
    call_static!(
		"edu/wpi/first/hal/HAL",
		"initialize",
		"(II)Z",
		&[JValue::Int(500).as_jni(),
          JValue::Int(1).as_jni()],
        jni::signature::ReturnType::Primitive(Primitive::Boolean)
    ).z().unwrap()
}

pub fn hal_report(resource: i32, instance_number: i32, context: i32, feature: String) {
    let string = java().new_string(feature).unwrap();
    call_static!(
		"edu/wpi/first/hal/HAL",
		"report",
		"(IIILjava/lang/String;)I",
		&[JValue::Int(resource).as_jni(),
		  JValue::Int(instance_number).as_jni(),
          JValue::Int(context).as_jni(),
          JValue::Object(&JObject::from_raw(string.into_raw())).as_jni()
        ],
        jni::signature::ReturnType::Primitive(Primitive::Int)
    ).i().unwrap();
}

pub struct AllianceStation(u8);

impl AllianceStation {
   pub fn red(&self) -> bool {
       match self.0 {
           1 | 2 | 3 => true,
           _ => false,
       }
   }
   pub fn blue(&self) -> bool {
       match self.0 {
           4 | 5 | 6 => true,
           _ => false,
       }
   }
}

pub fn alliance_station() -> AllianceStation {
    let station = call_static!(
		"frc/robot/Wrapper",
		"getAllianceStation",
		"()I",
        &Vec::new(),
        jni::signature::ReturnType::Primitive(Primitive::Int)
    ).i().unwrap();

    AllianceStation(station as u8)
}

pub async fn sleep_hz(mut instant: Instant, hz: i32) {
    let elapsed = instant.elapsed().as_secs_f64();
    let left = (1. / hz as f64 - elapsed).max(0.);
    tokio::time::sleep(Duration::from_secs_f64(left)).await;
    instant = Instant::now();
}

#[macro_export]
macro_rules! container {
    ($teleop:ident, $auto:ident, $($arg:expr),*) => {{
        let mut last_loop = std::time::Instant::now();

        loop {
            refresh_data();

            let state = RobotState::get();

            if state.enabled() && state.teleop() {
                $teleop($($arg),*).await;
            } else if state.enabled() && state.auto() {
                $auto($($arg),*).await;
            }
            
            sleep_hz(last_loop, 500).await;
        }
    }};
}

struct TaskId(String);

impl PartialEq for TaskId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for TaskId {}

impl Hash for TaskId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

pub struct TaskManager {
    running_tasks: HashMap<TaskId, (Arc<AtomicBool>, AbortHandle)>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            running_tasks: HashMap::new(),
        }
    }

    pub fn run_task<F, Fut>(&mut self, task_fn: F)
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        let type_id = format!("{:?}", TypeId::of::<F>());
        let task_id = TaskId(type_id);

        if !self.running_tasks.contains_key(&task_id) {
            // println!("Starting new task");

            let running = Arc::new(AtomicBool::new(true));
            let running_clone = running.clone();

            let task_fn = Arc::new(task_fn);
            let task_fn = task_fn.clone();

            let future = async move {
                // println!("Task loop starting");
                let mut interval = interval(Duration::from_millis(4));
                while running_clone.load(Ordering::SeqCst) {
                    interval.tick().await;
                    task_fn().await;
                }
                // println!("Task loop ended");
            };

            let abort_handle = spawn_local(future).abort_handle();

            self.running_tasks.insert(task_id, (running, abort_handle));
        } else {
            // println!("Task already running");
        }
    }

    pub fn abort_task<F, Fut>(&mut self, task_fn: F)
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        let type_id = format!("{:?}", TypeId::of::<F>());
        let task_id = TaskId(type_id);

        if let Some((running, abort_handle)) = self.running_tasks.remove(&task_id) {
            println!("Aborting task");
            running.store(false, Ordering::SeqCst);
            abort_handle.abort();
        } else {
            // println!("Task not found");
        }
    }
}

pub trait Robot {
    fn robot_init(&mut self);
    fn disabled_init(&mut self) {}
    fn autonomous_init(&mut self) {}
    fn teleop_init(&mut self) {}
    fn test_init(&mut self) {}

    fn disabled_periodic(&mut self) {}
    fn autonomous_periodic(&mut self) {}
    fn teleop_periodic(&mut self) {}
    fn test_periodic(&mut self) {}

    fn start_competition(&mut self, runtime: tokio::runtime::Runtime, local_set: LocalSet)
    where
        Self: 'static + Send + Sync,
    {
        runtime.block_on(local_set.run_until(async {
            self.robot_init();

            if !init_hal() {
                panic!("Failed to initialize HAL");
            }

            observe_user_program_starting();

            let mut previous_mode = RobotMode::Disabled;
            let mut last_loop = Instant::now();
            let mut dt = Duration::from_millis(0);

            loop {
                refresh_data();

                let state = RobotState::get().mode();

                // if state != previous_mode {
                //     if state == RobotMode::Disabled {
                //         self.disabled_init();
                //         previous_mode = RobotMode::Disabled;
                //     } else {
                //         match state {
                //             RobotMode::Auto => {
                //                 self.autonomous_init();
                //                 previous_mode = RobotMode::Auto;
                //             }
                //             RobotMode::Teleop => {
                //                 self.teleop_init();
                //                 previous_mode = RobotMode::Teleop;
                //             }
                //             RobotMode::Test => {
                //                 self.test_init();
                //                 previous_mode = RobotMode::Test;
                //             }
                //             RobotMode::Disabled => {}
                //         }
                //     }
                // }

                if state != previous_mode {
                    match state {
                        RobotMode::Disabled => {
                            self.disabled_init();
                            previous_mode = RobotMode::Disabled;
                        }
                        RobotMode::Auto => {
                            self.autonomous_init();
                            previous_mode = RobotMode::Auto;
                        }
                        RobotMode::Teleop => {
                            self.teleop_init();
                            previous_mode = RobotMode::Teleop;
                        }
                        RobotMode::Test => {
                            self.test_init();
                            previous_mode = RobotMode::Test;
                        }
                    }
                }

                match state {
                    RobotMode::Disabled => self.disabled_periodic(),
                    RobotMode::Auto => self.autonomous_periodic(),
                    RobotMode::Teleop => self.teleop_periodic(),
                    RobotMode::Test => self.test_periodic(),
                }


                previous_mode = state.clone();

                // Enforce a periodic loop delay
                dt = last_loop.elapsed();
                let elapsed = dt.as_secs_f64();
                let left = (1. / 250. - elapsed).max(0.);
                sleep(Duration::from_secs_f64(left)).await;
                last_loop = Instant::now();
            }
        }));
    }
}
