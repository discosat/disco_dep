use crate::slash_sys::*;

unsafe impl Sync for slash_command {}

/// Force everything here to be loaded by linker
/// This only works because it is being used in the header file
#[no_mangle]
pub static RUST_SLASH_LOAD: u32 = 0;

/// Register a new command to slash.
///
/// This takes the same arguments as __slash_command in slash
#[macro_export]
macro_rules! register_slash_command {
    ($ident:ident, $name:literal, $func:expr, $completer:expr, $args:literal) => {
        #[link_section = "slash"]
        #[used]
        pub static $ident: crate::slash_sys::slash_command = crate::slash_sys::slash_command {
            name: concat!($name, "\0").as_ptr() as *mut std::os::raw::c_char,
            func: $func,
            args: concat!($args, "\0").as_ptr() as *const std::os::raw::c_char,
            completer: $completer
        };
    }
}

register_slash_command!{helloa, "Helloa", None, None, ""}
register_slash_command!{hellob, "Hellob", None, None, ""}
register_slash_command!{helloc, "Helloc", None, None, ""}