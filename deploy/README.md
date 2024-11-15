# Deploy Crate

## Introduction

The Deploy Crate is a  tool to simplify the deployment process of your Rust code to a robot.

## Installation

To get started with the Deploy Crate, you need to clone the repository and install

```bash
git clone https://github.com/Team-2502/frcrs.git
cd frcrs/deploy
cargo install --path .
```

You will also need to add a deploy section to you Cargo.toml
```toml
[deploy]
team_number = 2502
executable = "target/arm-unknown-linux-gnueabi/release/robotcode"
lib = "javastub.jar"
frontend = "talon-board/out"
frontend_dest = "/home/lvuser/talon-board/out"
```

You can now use the command line to deploy
```bash
deploy -b -u -l -f
```
The arguments include:
* -b: build robotcode
* -u: deploy over usb ip
* -l: copy javastub library
* -f: copy frontend