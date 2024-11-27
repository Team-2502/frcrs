use crate::edu_wpi_first_hal_HAL;

pub struct Hal;

impl Hal {
    pub fn initialize(timeout: i32, mode: i32) -> bool {
        edu_wpi_first_hal_HAL::initialize(timeout, mode).unwrap()
    }

    pub fn shutdown() {
        edu_wpi_first_hal_HAL::shutdown().unwrap()
    }
}