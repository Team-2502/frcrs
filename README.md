# frcrs
frcrs is a robotics control framework designed to streamline the development of control systems for FRC robots. It provides a highly modular architecture, focusing on creating independent and reusable subsystems to simplify the overall control structure of the robot.

# Features
- Modular Subsystem Structure: Each robot subsystem (e.g., Drivetrain, Shooter) is separated into individual modules, promoting clean, maintainable code.
- Easy Integration: Easily integrate into a runnable robot, will build with no hassle
- Asynchronous: Asynchronous tasks, allowing efficient control of subsystems in real-time scenarios.
# Subsystem Overview
The main design principle of frcrs is to split robot functionality into independent subsystems. Each subsystem is responsible for controlling a specific part of the robot, such as the drivetrain, shooter, or intake mechanism.

- Subsystems: Implemented using Rust structs, subsystems encapsulate the hardware interface and logic for a specific component of the robot.
- Controllers: Subsystems are controlled via Joystick or other input mechanisms, which can be passed into the subsystems as needed.
- Communication: Subsystems communicate with each other through shared ownership patterns, using Rc<RefCell<>> to manage state in a safe and efficient way.
# Example Usage
Here’s an example of how a subsystem might be implemented in frcrs:

```rust
use frcrs::ctre::SRX;
use crate::constants;

pub struct Drivetrain {
    left: SRX,
    right: SRX
}

impl Drivetrain {
    pub fn new() -> Self {
        Self {
            left: SRX::new(constants::drivetrain::LEFT),
            right: SRX::new(constants::drivetrain::RIGHT)
        }
    }

    pub fn drive(&self, left: f64, right: f64) {
        self.left.set(-left);
        self.right.set(-right);
    }
}
```
See an example [here](https://github.com/Team-2502/frcrs-template/tree/disco)
# Getting Started
1. Clone the Repository
    ```bash
    git clone https://github.com/Team-2502/frcrs.git
    cd frcrs
    ```
2. Ensure that you have Rust and cargo installed. You can install Rust [here](https://www.rust-lang.org/tools/install)
3. Install Java 17, the same for FRC
4. Install rust target
   ```bash
   rustup target add arm-unknown-linux-gnueabi
   ```
6. Install toolchain
   ```bash
   git clone https://github.com/wpilibsuite/allwpilib.git
   cd allwpipib
   ./gradlew installRoborioToolchain
   ```
7. Add directory to path `$HOME/.gradle/toolchains/frc/YYYY/roborio/bin` where `YYYY` is the year of the toolchain
8. Build frcrs with `cargo build --release --target arm-unknown-linux-gnueabi`
9. Create robotcode, see an example [here](https://github.com/Team-2502/frcrs-template/tree/disco)
10. Deploy your code, see deploy crate for deployment. An example toml is here
    ```toml
    [deploy]
    team_number = 2502
    executable = "target/arm-unknown-linux-gnueabi/release\\robotcode"
    lib = "javastub.jar"
    frontend = "talon-board/out"
    frontend_dest = "/home/lvuser/talon-board/out"
    ```

# Contributing
We welcome contributions! If you have suggestions for improvements or find bugs, feel free to create an issue or submit a pull request.

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a pull request

# License
This project is licensed under the MIT License - see the LICENSE file for details.
