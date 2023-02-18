#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
pub mod slash_sys;
mod slash;
mod slash_exit_code;

pub use slash_exit_code::*;
pub use slash::*;
pub use slash_sys::*;