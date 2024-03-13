
use navx_sys::AHRS;

pub struct NavX {
    navx: *mut AHRS,
}

impl NavX {
    pub fn new() -> Self {
        let navx = unsafe {navx_sys::navx_wrapper_bind_navx_mxp()};

        Self {
            navx
        }
    }

    pub fn get_angle(&self) -> f64 {
        unsafe {navx_sys::AHRS_GetAngle(self.navx)}
    }

    pub fn reset_angle(&self) {
        unsafe {navx_sys::AHRS_Reset(self.navx)}
        
    }
}
