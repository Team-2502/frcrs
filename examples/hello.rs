use std::thread::sleep;
use std::time::Duration;
use frcrs::{DriverStation, Hal};

fn main() {
    if !Hal::initialize(500, 0) {
        panic!("Failed to initialize HAL");
    }

    DriverStation::observe_user_program_starting();

    loop {
        DriverStation::refresh_data();

        println!("Hello!");

        sleep(Duration::from_secs(1));
    }
}