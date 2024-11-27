pub struct Hal;

impl Hal {
    pub fn initialize(timeout: i32, mode: i32) -> bool {
        hal_sys::edu_wpi_first_hal_HAL::initialize(timeout, mode).unwrap()
    }

    pub fn shutdown() {
        hal_sys::edu_wpi_first_hal_HAL::shutdown().unwrap()
    }
}