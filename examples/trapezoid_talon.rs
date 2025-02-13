use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use tokio::task::LocalSet;
use tokio::time::sleep;
use frcrs::{observe_user_program_starting, refresh_data};
use frcrs::ctre::{ControlMode, Talon};
use frcrs::input::RobotState;
use frcrs::trapezoidal::{TrapezoidalProfile, PID};

fn main() {
    let executor = Runtime::new().unwrap();
    let local = LocalSet::new();

    let controller = local.run_until(async {
        observe_user_program_starting();

        let mut talon = Talon::new(1, None);

        // Define motion profile parameters
        let start_position = talon.get_position();
        let goal_position = 100.0;
        let max_velocity = 50.0;     // units per second (for example, inches/sec or degrees/sec)
        let max_acceleration = 5.0; // units per second^2

        // Create the trapezoidal profile
        let mut profile = TrapezoidalProfile::new(start_position, goal_position, max_velocity, max_acceleration);

        // Create a PID controller (tune gains as appropriate)
        let mut pid = PID::new(0.005, 0.0, 0.001);

        println!("Time\tDesired\tMotor Pos\tPID Output\tFeedforward Velocity");

        let mut last_loop = Instant::now();
        let mut dt = Duration::from_millis(0);

        loop {
            refresh_data();

            let state = RobotState::get();

            if state.enabled() && state.teleop() {
                let (desired_position, desired_velocity) = profile.update(dt.as_secs_f64());

                // The PID controller computes the output (percent output, e.g., -1.0 to 1.0)
                let pid_output = pid.update(desired_position, talon.get_position(), dt.as_secs_f64());

                talon.set(ControlMode::Percent, pid_output);

                println!(
                    "time: {} desired: {:.2} current: {:.2} pid: {:.2} des velo: {:.2}",
                    dt.as_secs_f64(), desired_position, talon.get_position(), pid_output, desired_velocity
                );
            } else {
                talon.stop();
            }

            dt = last_loop.elapsed();
            let elapsed = dt.as_secs_f64();
            let left = (1. / 250. - elapsed).max(0.);
            sleep(Duration::from_secs_f64(left)).await;
            last_loop = Instant::now();
        }
    });

    executor.block_on(controller);
}