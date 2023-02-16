use crate::slash_sys::slash_command;

unsafe impl Sync for slash_command {}

/// Register a new command to slash.
///
/// This takes the same arguments as __slash_command in slash
// #[macro_export]
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
pub(crate) use register_slash_command;