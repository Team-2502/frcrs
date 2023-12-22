#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum Error {
    #[error("The CAN message was stale")]
    CanMsgStale = 1,
    #[error("A can message could be not transmitted")]
    CanTransmissionError = -1,
    #[error("Caller passed an invalid param")]
    InvalidParamValue = -2,
    #[error("CAN frame could not be recieved properly")]
    RxError = -3,
    #[error("A transmisson error occured")]
    TxError = -4,
    #[error("Specified device id is invalid or no more can sessions avaliable")]
    CanIdError = -5,
    #[error("CAN buffer was full and message could not be sent")]
    CanBufferFull = 6,
    #[error("The CAN bus has overflowed")]
    CanOverflow = -6,
    #[error("Sensor is not present")]
    SensorNotPresent = -7,
    #[error("Device firmware is too old, please update")]
    FirmwareTooOld = -8,
    #[error("Could not change period")]
    CouldNotChangePeriod = -9,
    #[error("CAN buffer has failed")]
    BufferFailure = -10,
    #[error("Firmware was of incorect type")]
    FirmwareNonFRC = -11,
    #[error("A general error has occured")]
    GeneralError = -100,
    #[error("Have not received an value response for signal")]
    SigNotUpdated = -200,
    #[error("Not all PID values where updated (signal error)")]
    NotAllPIDValuesUpdated = -201,
    #[error("General Port Error")]
    GenPortError = -300,
    #[error("Port Module Type Mismatch")]
    PortModuleTypeMismatch = -301,
    #[error("General Module Error")]
    GenModuleError = -400,
    #[error("Module Not Initialized (Set)")]
    ModuleNotInitSetError = -401,
    #[error("Module Not Initialized (Get)")]
    ModuleNotInitGetError = -402,
    #[error("Wheel Radius Too Small")]
    WheelRadiusTooSmall = -500,
    #[error("Ticks Per Revolution is Zero")]
    TicksPerRevZero = -501,
    #[error("Distance Between Wheels Too Small")]
    DistanceBetweenWheelsTooSmall = -502,
    #[error("Gains Are Not Set")]
    GainsAreNotSet = -503,
    #[error("Wrong Remote Limit Switch Source")]
    WrongRemoteLimitSwitchSource = -504,
    #[error("Double Voltage Compensating WPI")]
    DoubleVoltageCompensatingWPI = -505,
    #[error("CANdle Animation Slot Out of Bounds")]
    CANdleAnimSlotOutOfBounds = -506,
    #[error("Incompatible Mode")]
    IncompatibleMode = -600,
    #[error("Invalid Handle")]
    InvalidHandle = -601,
    #[error("Feature Requires Higher Firmware Version")]
    FeatureRequiresHigherFirm = -700,
    #[error("Motor Controller Feature Requires Higher Firmware Version")]
    MotorControllerFeatureRequiresHigherFirm = -701,
    #[error("Config Factory Default Requires Higher Firmware Version")]
    ConfigFactoryDefaultRequiresHigherFirm = -702,
    #[error("Config Motion S-Curve Requires Higher Firmware Version")]
    ConfigMotionSCurveRequiresHigherFirm = -703,
    #[error("TalonFX Firmware Pre-VBat Detect")]
    TalonFXFirmwarePreVBatDetect = -704,
    #[error("CANdle Animations Require Higher Firmware Version")]
    CANdleAnimationsRequireHigherFirm = -705,
    #[error("Library Could Not Be Loaded")]
    LibraryCouldNotBeLoaded = -800,
    #[error("Missing Routine in Library")]
    MissingRoutineInLibrary = -801,
    #[error("Resource Not Available")]
    ResourceNotAvailable = -802,
    #[error("Music File Not Found")]
    MusicFileNotFound = -900,
    #[error("Music File Wrong Size")]
    MusicFileWrongSize = -901,
    #[error("Music File Too New")]
    MusicFileTooNew = -902,
    #[error("Music File Invalid")]
    MusicFileInvalid = -903,
    #[error("Invalid Orchestra Action")]
    InvalidOrchestraAction = -904,
    #[error("Music File Too Old")]
    MusicFileTooOld = -905,
    #[error("Music Interrupted")]
    MusicInterrupted = -906,
    #[error("Music Not Supported")]
    MusicNotSupported = -907,
    #[error("Invalid Interface")]
    InvalidInterface = -1000,
    #[error("Invalid GUID")]
    InvalidGuid = -1001,
    #[error("Invalid Class")]
    InvalidClass = -1002,
    #[error("Invalid Protocol")]
    InvalidProtocol = -1003,
    #[error("Invalid Path")]
    InvalidPath = -1004,
    #[error("General WinUsb Error")]
    GeneralWinUsbError = -1005,
    #[error("Failed Setup")]
    FailedSetup = -1006,
    #[error("Listen Failed")]
    ListenFailed = -1007,
    #[error("Send Failed")]
    SendFailed = -1008,
    #[error("Receive Failed")]
    ReceiveFailed = -1009,
    #[error("Invalid Response Format")]
    InvalidRespFormat = -1010,
    #[error("WinUsb Init Failed")]
    WinUsbInitFailed = -1011,
    #[error("WinUsb Query Failed")]
    WinUsbQueryFailed = -1012,
    #[error("WinUsb General Error")]
    WinUsbGeneralError = -1013,
    #[error("Access Denied")]
    AccessDenied = -1014,
    #[error("Firmware Invalid Response")]
    FirmwareInvalidResponse = -1015,
    #[error("Pulse Width Sensor Not Present")]
    PulseWidthSensorNotPresent = 10,
    #[error("General Warning")]
    GeneralWarning = 100,
    #[error("Feature Not Supported")]
    FeatureNotSupported = 101,
    #[error("Not Implemented")]
    NotImplemented = 102,
    #[error("Firmware Version Could Not Be Retrieved")]
    FirmVersionCouldNotBeRetrieved = 103,
    #[error("Features Not Available Yet")]
    FeaturesNotAvailableYet = 104,
    #[error("Control Mode Not Valid")]
    ControlModeNotValid = 105,
    #[error("Control Mode Not Supported Yet")]
    ControlModeNotSupportedYet = 106,
    #[error("PID Type Not Supported Yet")]
    PIDTypeNotSupportedYet = 107,
    #[error("Remote Sensors Not Supported Yet")]
    RemoteSensorsNotSupportedYet = 108,
    #[error("Motion Profile Firmware Threshold")]
    MotProfFirmThreshold = 109,
    #[error("Motion Profile Firmware Threshold 2")]
    MotProfFirmThreshold2 = 110,
    #[error("Sim Device Not Found")]
    SimDeviceNotFound = 200,
    #[error("Sim Physics Type Not Supported")]
    SimPhysicsTypeNotSupported = 201,
    #[error("Sim Device Already Exists")]
    SimDeviceAlreadyExists = 202,
}

pub fn to_result_with_value<T>(code: i32, value: T) -> Result<T, Error> {
    if code == 0 {
        Ok(value)
    } else {
        Err(match code {
            1 => Error::CanMsgStale,
            -1 => Error::CanTransmissionError,
            -2 => Error::InvalidParamValue,
            -3 => Error::RxError,
            -4 => Error::TxError,
            -5 => Error::CanIdError,
            6 => Error::CanBufferFull,
            -6 => Error::CanOverflow,
            -7 => Error::SensorNotPresent,
            -8 => Error::FirmwareTooOld,
            -9 => Error::CouldNotChangePeriod,
            -10 => Error::BufferFailure,
            -11 => Error::FirmwareNonFRC,
            -100 => Error::GeneralError,
            -200 => Error::SigNotUpdated,
            -201 => Error::NotAllPIDValuesUpdated,
            -300 => Error::GenPortError,
            -301 => Error::PortModuleTypeMismatch,
            -400 => Error::GenModuleError,
            -401 => Error::ModuleNotInitSetError,
            -402 => Error::ModuleNotInitGetError,
            -500 => Error::WheelRadiusTooSmall,
            -501 => Error::TicksPerRevZero,
            -502 => Error::DistanceBetweenWheelsTooSmall,
            -503 => Error::GainsAreNotSet,
            -504 => Error::WrongRemoteLimitSwitchSource,
            -505 => Error::DoubleVoltageCompensatingWPI,
            -506 => Error::CANdleAnimSlotOutOfBounds,
            -600 => Error::IncompatibleMode,
            -601 => Error::InvalidHandle,
            -700 => Error::FeatureRequiresHigherFirm,
            -701 => Error::MotorControllerFeatureRequiresHigherFirm,
            -702 => Error::ConfigFactoryDefaultRequiresHigherFirm,
            -703 => Error::ConfigMotionSCurveRequiresHigherFirm,
            -704 => Error::TalonFXFirmwarePreVBatDetect,
            -705 => Error::CANdleAnimationsRequireHigherFirm,
            -800 => Error::LibraryCouldNotBeLoaded,
            -801 => Error::MissingRoutineInLibrary,
            -802 => Error::ResourceNotAvailable,
            -900 => Error::MusicFileNotFound,
            -901 => Error::MusicFileWrongSize,
            -902 => Error::MusicFileTooNew,
            -903 => Error::MusicFileInvalid,
            -904 => Error::InvalidOrchestraAction,
            -905 => Error::MusicFileTooOld,
            -906 => Error::MusicInterrupted,
            -907 => Error::MusicNotSupported,
            -1000 => Error::InvalidInterface,
            -1001 => Error::InvalidGuid,
            -1002 => Error::InvalidClass,
            -1003 => Error::InvalidProtocol,
            -1004 => Error::InvalidPath,
            -1005 => Error::GeneralWinUsbError,
            -1006 => Error::FailedSetup,
            -1007 => Error::ListenFailed,
            -1008 => Error::SendFailed,
            -1009 => Error::ReceiveFailed,
            -1010 => Error::InvalidRespFormat,
            -1011 => Error::WinUsbInitFailed,
            -1012 => Error::WinUsbQueryFailed,
            -1013 => Error::WinUsbGeneralError,
            -1014 => Error::AccessDenied,
            -1015 => Error::FirmwareInvalidResponse,
            10 => Error::PulseWidthSensorNotPresent,
            100 => Error::GeneralWarning,
            101 => Error::FeatureNotSupported,
            102 => Error::NotImplemented,
            103 => Error::FirmVersionCouldNotBeRetrieved,
            104 => Error::FeaturesNotAvailableYet,
            105 => Error::ControlModeNotValid,
            106 => Error::ControlModeNotSupportedYet,
            107 => Error::PIDTypeNotSupportedYet,
            108 => Error::RemoteSensorsNotSupportedYet,
            109 => Error::MotProfFirmThreshold,
            110 => Error::MotProfFirmThreshold2,
            200 => Error::SimDeviceNotFound,
            201 => Error::SimPhysicsTypeNotSupported,
            202 => Error::SimDeviceAlreadyExists,
            x => {
                tracing::error!("Unknown ctre error code {x}");
                panic!("Unknown revlib error code {x}");
            }
        })
    }
}

pub fn to_result(code: i32) -> Result<(), Error> {
    to_result_with_value(code, ())
}
