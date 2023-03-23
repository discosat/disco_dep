use thiserror::Error;
use crate::SlashExitCode;

pub type SlashResult<T> = Result<T, SlashError>;

#[derive(Error, Debug)]
pub enum SlashError {
    #[error(transparent)]
    NulError(#[from] std::ffi::NulError),

    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Recieved invalid character '{failed_on}' ")]
    InvalidCharacter { failed_on: char },

    #[error("Failed with exit code {0:?}")]
    ExitCode(SlashExitCode),
}

impl From<SlashError> for SlashExitCode {
    fn from(value: SlashError) -> Self {
        match value {
            SlashError::ExitCode(code) => code,
            _ => SlashExitCode::Exit
        }
    }
}