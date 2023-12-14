use std::{ffi::CStr, fmt::Display};

use hal_sys::HAL_GetErrorMessage;

#[derive(Debug)]
pub struct HalError(pub(crate) i32);

impl HalError {
    pub fn from_raw(raw: i32) -> Self {
        HalError(raw)
    }
}

impl Display for HalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match unsafe { CStr::from_ptr(HAL_GetErrorMessage(self.0)).to_str() } {
                Ok(res) => res,
                Err(_) => {
                    return Err(std::fmt::Error);
                }
            }
        )
    }
}

impl std::error::Error for HalError {}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The HAL encountered an error: {0}")]
    HalError(#[from] HalError),
    #[error("The notifier was stopped")]
    NotifierStopped,
    #[error("The button {0} was out of range")]
    ButtonIndexOutOfRange(u32),
    #[error("The joystick {0} was out of range")]
    JoystickIndexOutOfRange(u32),
    #[error("The axis {0} was out of range")]
    AxisIndexOutOfRange(u32),
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! status_to_result {
    ($func:ident($($param:expr),+)) => {
        {
            let mut status = 0;
            let res = $func($($param),*, &mut status);

            if status == 0 {
                Ok(res)
            } else {
                Err(crate::error::HalError(status))
            }
        }
    };
    ($func:ident()) => {
        {
            let mut status = 0;
            let res = $func(&mut status);

            if status == 0 {
                Ok(res)
            } else {
                Err(crate::error::HalError(status))
            }
        }
    };
}
