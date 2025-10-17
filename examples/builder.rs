use frcrs::input::{Gamepad, Joystick};
use frcrs::{observe_user_program_starting, refresh_data};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::task::LocalSet;

//#[tokio::main(flavor = "current_thread")]
fn main() {
    let executor = Runtime::new().unwrap();
    let local = LocalSet::new();

    let controller = local.run_until(async {
        let mut joystick = Gamepad::new(1);

        observe_user_program_starting();

        loop {
            refresh_data();

            // if joystick.a() {
            //     println!("A pressed");
            // }

            joystick.clone().while_held(1, &local, || async {
                println!("Button 1 pressed");
                tokio::time::sleep(Duration::from_secs(1)).await;
                println!("Button 1 finished");
            });

            joystick.clone().while_held(2, &local, || async {
                println!("Button 2 pressed");
                tokio::time::sleep(Duration::from_secs(1)).await;
                println!("Button 2 finished");
            });
        }
    });

    executor.block_on(controller);
}

// async fn test() {
//
// }
//
// struct Ferris {
//     drivetrain: Drivetrain
// }
//
// impl Robot for Ferris {
//     async fn initialize() -> Self {
//         println!("Initializing robot");
//         Self
//     }
//
//     async fn auto(&self) {
//         println!("Running auto");
//     }
//
//     async fn teleop(&self) {
//         println!("Running teleop");
//     }
//
//     async fn test(&self) {
//         println!("Running test");
//     }
//
//     async fn disabled(&self) {
//         println!("Robot disabled");
//     }
// }
//
// struct Drivetrain;
//
// impl Drivetrain {
//     pub fn new() -> Self {
//         Self
//     }
//
//     pub fn drive(&self) {
//         println!("Driving");
//     }
// }
//
