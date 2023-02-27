use super::slash_sys::*;
use std::os::raw::c_int;

pub enum SlashExitCode {
    Exit,
    Success,
    Eusage,
    Einval,
    Enospc,
    Eio,
    Enomem,
    Enoent
}

impl SlashExitCode {
    pub fn code(&self) -> c_int {
        match self {
            SlashExitCode::Exit => SLASH_EXIT as c_int,
            SlashExitCode::Success => SLASH_SUCCESS as c_int,
            SlashExitCode::Eusage => SLASH_EUSAGE as c_int,
            SlashExitCode::Einval => SLASH_EINVAL as c_int,
            SlashExitCode::Enospc => SLASH_ENOSPC as c_int,
            SlashExitCode::Eio => SLASH_EIO as c_int,
            SlashExitCode::Enomem => SLASH_ENOMEM as c_int,
            SlashExitCode::Enoent => SLASH_ENOENT as c_int,
        }
    }
}