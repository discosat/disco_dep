use thiserror::Error;

pub type CSPResult<T> = Result<T, CSPError>;

#[derive(Error, Debug)]
pub enum CSPError {
    #[error("Failed to allocate buffer")]
    FailedToAllocBuffer,
    #[error("Failed to connect")]
    ConnectionFailed,
    #[error("Failed to convert from null pointer")]
    NullPointerCast,
    #[error("Failed to convert from pointer")]
    FailedPointerCast,

    #[error("Not enough memory")]
    Nomem,
    #[error("Invalid argument")]
    Inval,
    #[error("Operation timed out")]
    Timedout,
    #[error("Resource already in use")]
    Used,
    #[error("Operation not supported")]
    Notsup,
    #[error("Device or resource busy")]
    Busy,
    #[error("Connection already in progress")]
    Already,
    #[error("Connection reset")]
    Reset,
    #[error("No more buffer space available")]
    Nobufs,
    #[error("Transmission failed")]
    Tx,
    #[error("Error in driver layer")]
    Driver,
    #[error("Resource temporarily unavailable")]
    Again,
    #[error("Function not implemented")]
    Nosys,
    #[error("HMAC failed")]
    Hmac,
    #[error("CRC32 failed")]
    Crc32,
    #[error("SFP protocol error or inconsistency")]
    Sfp,
    #[error("Unknown error code")]
    UnknownErrorCode
}

impl CSPError {
    pub fn from_int(err: i32) -> Result<(), CSPError> {
        match err {
            -1 => Err(CSPError::Nomem),
            -2 => Err(CSPError::Inval),
            -3 => Err(CSPError::Timedout),
            -4 => Err(CSPError::Used),
            -5 => Err(CSPError::Notsup),
            -6 => Err(CSPError::Busy),
            -7 => Err(CSPError::Already),
            -8 => Err(CSPError::Reset),
            -9 => Err(CSPError::Nobufs),
            -10 => Err(CSPError::Tx),
            -11 => Err(CSPError::Driver),
            -12 => Err(CSPError::Again),
            -38 => Err(CSPError::Nosys),
            -100 => Err(CSPError::Hmac),
            -102 => Err(CSPError::Crc32),
            -103 => Err(CSPError::Sfp),
            0 => Ok(()),
            _ => Err(CSPError::UnknownErrorCode)
        }
    }
}