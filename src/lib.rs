include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod hal;
mod driverstation;
mod ctre;

pub use hal::Hal;
pub use driverstation::DriverStation;
pub use ctre::*;