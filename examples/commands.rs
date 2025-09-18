use frcrs::input::Gamepad;
use frcrs::{Robot, TaskManager};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::task::{spawn_local, LocalSet};
use tokio::time::sleep;

pub struct MyRobot {
    task_manager: TaskManager,
    gamepad: Gamepad,
}

impl MyRobot {
    pub fn new() -> Self {
        Self {
            task_manager: TaskManager::new(),
            gamepad: Gamepad::new(1),
        }
    }
}

impl Robot for MyRobot {
    fn robot_init(&mut self) {
        println!("Robot init");
    }

    fn disabled_init(&mut self) {
        println!("Disabled init");
    }

    fn autonomous_init(&mut self) {
        println!("Autonomous init");
    }

    fn teleop_init(&mut self) {
        println!("Teleop init");
    }

    fn test_init(&mut self) {
        println!("Test init");
    }

    async fn disabled_periodic(&mut self) {
        // println!("Disabled periodic");
    }

    async fn autonomous_periodic(&mut self) {
        // println!("Autonomous periodic");
    }

    async fn teleop_periodic(&mut self) {
        let task = || async {
            println!("Task Started");
            sleep(Duration::from_secs(1)).await;
            println!("Task Finished");
        };

        if self.gamepad.left_bumper() {
            self.task_manager.run_task(task);
        } else {
            self.task_manager.abort_task(task);
        }
    }

    async fn test_periodic(&mut self) {
        println!("Test periodic");
    }
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let local_set = LocalSet::new();

    let mut my_robot = MyRobot::new();
    my_robot.start_competition(runtime, local_set);
}
