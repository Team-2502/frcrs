use thiserror::Error;
use tracing::error;

#[macro_export]
macro_rules! handle_error {
    ($($err:tt)+) => {{
        match $($err)+ {
            0 => Ok(()),
            val => Err(crate::error::REVError::from(val)),
        }
    }};
}

#[derive(Debug, Clone, Error)]
pub enum REVError {
    #[error("General error")]
    General = 1,
    #[error("Timed out while waiting for the CAN")]
    CANTimeout = 2,
    #[error("Function or feature not implemented by REV")]
    NotImplemented = 3,
    #[error("WPILib or external HAL error")]
    HAL = 4,
    #[error("Unable to retrieve SPARK MAX firmware version. Please verify the deviceID field matches the configured CAN ID of the controller, and that the controller is connected to the CAN Bus.")]
    CantFindFirmware = 5,
    #[error("The firmware is too old and needs to be updated. Refer to www.revrobotics.com/REVLib for details.")]
    FirmwareTooOld = 6,
    #[error("The library version is outdated. If this is the latest release please file an issue")]
    FirmwareTooNew = 7,
    #[error("Invalid parameter id")]
    ParamInvalidID = 8,
    #[error("Parameter type mismatch for parameter id")]
    ParamMismatchType = 9,
    #[error("Invalid parameter access mode parameter id")]
    ParamAccessMode = 10,
    #[error("Received parameter invalid error parameter id")]
    ParamInvalid = 11,
    #[error("Parameter is either not implemented or has been deprecated id ")]
    ParamNotImplementedDeprecated = 12,
    #[error("Follower config setup check failed, check follower mode is set properly on device!")]
    FollowConfigMismatch = 13,
    #[error("Error invalid")]
    Invalid = 14,
    #[error("Setpoint is out of the defined range")]
    SetpointOutOfRange = 15,
    #[error("Unknown error (this error is unknown to the revlib drivers not to the rust crate)")]
    Unknown = 16,
    #[error("CAN Output Buffer Full. Ensure a device is attached to the CAN bus.")]
    CANDisconnected = 17,
    #[error("A CANREVLib object with this ID was already created.")]
    DuplicateCANId = 18,
    #[error("A CANREVLib object was given an invalid CAN ID.")]
    InvalidCANId = 19,
    #[error("The spark max data port was already configured in a different way")]
    SparkMaxDataPortAlreadyConfiguredDifferently = 20,
}

impl From<u32> for REVError {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::General,
            2 => Self::CANTimeout,
            3 => Self::NotImplemented,
            4 => Self::HAL,
            5 => Self::CantFindFirmware,
            6 => Self::FirmwareTooOld,
            7 => Self::FirmwareTooNew,
            8 => Self::ParamInvalidID,
            9 => Self::ParamMismatchType,
            10 => Self::ParamAccessMode,
            11 => Self::ParamInvalid,
            12 => Self::ParamNotImplementedDeprecated,
            13 => Self::FollowConfigMismatch,
            14 => Self::Invalid,
            15 => Self::SetpointOutOfRange,
            16 => Self::Unknown,
            17 => Self::CANDisconnected,
            18 => Self::DuplicateCANId,
            19 => Self::InvalidCANId,
            20 => Self::SparkMaxDataPortAlreadyConfiguredDifferently,
            code => {
                error!("Unknown rev error code {code}, exiting");
                panic!("Unknown rev error code {code}");
            }
        }
    }
}
