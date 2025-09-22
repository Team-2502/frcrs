use frcrs::input::Joystick;
use frcrs::{observe_user_program_starting, refresh_data};
use tokio::runtime::Runtime;
use tokio::task::LocalSet;

fn main() {
    let executor = Runtime::new().unwrap();
    let local = LocalSet::new();

    let controller = local.run_until(async {
        let mut joystick = Joystick::new(0);

        observe_user_program_starting();

        loop {
            refresh_data();

            println!("POV: {}", joystick.get_pov());
        }
    });

    executor.block_on(controller);
}
