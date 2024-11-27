pub struct DriverStation;

impl DriverStation {
    pub fn refresh_data() {
        hal_sys::edu_wpi_first_wpilibj_DriverStation::refreshData().unwrap()
    }

    pub fn observe_user_program_starting() {
        hal_sys::edu_wpi_first_hal_DriverStationJNI::observeUserProgramStarting().unwrap()
    }
}