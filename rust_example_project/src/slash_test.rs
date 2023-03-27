use std::process::ExitCode;

use crate::slash_sys::*;

pub fn hello_rust(slash: &Slash) -> Result<(), SlashExitCode> {
    println!("Hello user glad to talk to you!");
    Ok(())
}

register_slash_command!{hello_rust, "hello_rust", None, ""}