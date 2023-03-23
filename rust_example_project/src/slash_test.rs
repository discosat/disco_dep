use std::process::ExitCode;

use crate::slash_sys::slash_sys::slash;
use crate::slash_sys::*;
use crate::slash_sys::register_slash_command;

pub fn hello_rust(slash: &slash) -> Result<(), SlashExitCode> {
    println!("Hello user glad to talk to you!");
    Ok(())
}

register_slash_command!{hello_rust, "hello_rust", None, ""}