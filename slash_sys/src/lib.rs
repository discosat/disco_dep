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