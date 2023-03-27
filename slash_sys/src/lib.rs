#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
pub mod slash_sys;

pub use paste;
mod slash;
mod slash_exit_code;
mod arg_parse;
mod slash_error;

pub use slash_error::*;
pub use arg_parse::*;
pub use slash_exit_code::*;
pub use slash::*;
use slash_sys::{slash_dfl_node, slash_dfl_timeout};

pub fn default_node() -> u16 {
    unsafe {
        slash_dfl_node as u16
    }
}

pub fn default_timout() -> u32 {
    unsafe {
        slash_dfl_timeout
    }
}