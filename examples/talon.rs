use frcrs::{DriverStation, Hal, TalonFX};

fn main() {
    if !Hal::initialize(500, 0) {
        panic!("Failed to initialize HAL");
    }

    DriverStation::observe_user_program_starting();

    let talon = TalonFX::new(1, None);

    loop {
        DriverStation::refresh_data();

        if DriverStation::teleop() && DriverStation::enabled() {
            talon.set(0.1);
        } else {
            talon.stop();
        }
    }
}