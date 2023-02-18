use crate::register_slash_command;
use crate::slash_sys::*;
use std::os::raw::c_int;
use crate::slash::*;

/// Force everything here to be loaded by linker
/// This works because it is being used in the header file
/// For files with other extern functions this might not be necessary
#[no_mangle]
pub static RUST_SLASH_LOAD: u32 = 0;


fn hello_rust(slash: &slash) -> SlashExitCode {
    println!("Hello user glad to talk to you!");
    SlashExitCode::Success
}

register_slash_command!{hello_rust, "Hello rust there", None, ""}