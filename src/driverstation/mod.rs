use crate::{edu_wpi_first_hal_DriverStationJNI, edu_wpi_first_wpilibj_DriverStation};

pub struct DriverStation;

impl DriverStation {
    pub fn refresh_data() {
        edu_wpi_first_wpilibj_DriverStation::refreshData().unwrap()
    }

    pub fn observe_user_program_starting() {
        edu_wpi_first_hal_DriverStationJNI::observeUserProgramStarting().unwrap()
    }

    pub fn enabled() -> bool {
        edu_wpi_first_wpilibj_DriverStation::isEnabled().unwrap()
    }

    pub fn teleop() -> bool {
        edu_wpi_first_wpilibj_DriverStation::isTeleop().unwrap()
    }
}