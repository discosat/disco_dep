use crate::slash_sys::slash_command;

unsafe impl Sync for slash_command {}

/// Register a new command to slash.
///
/// This takes the same arguments as __slash_command in slash
macro_rules! register_slash_command {
    // Create an extern wrapper around a command
    (_create_extern_command_wrapper $name:ident) => {
        paste::paste!{
            #[no_mangle]
            pub unsafe extern "C" fn [<extern_$name>] (slash: *mut slash) -> c_int {
                if let Some(slash_ref) = slash.as_ref() {
                    let res: crate::SlashExitCode = $name(slash_ref);
                    res.code()
                } else {
                    crate::SlashExitCode::Exit.code()
                }
            }
        }
        
    };

    // Register the command with slash
    (_register_command $ident:ident, $name:literal, $completer:expr, $args:literal) => {
        paste::paste!{
            #[link_section = "slash"]
            #[used]
            pub static [<static_$ident>]: crate::slash_sys::slash_command = crate::slash_sys::slash_command {
                name: concat!($name, "\0").as_ptr() as *mut std::os::raw::c_char,
                func: Some([<extern_$ident>]),
                args: concat!($args, "\0").as_ptr() as *const std::os::raw::c_char,
                completer: $completer
            };
        }
    };

    // Takes arguments from user
    ($ident:ident, $name:literal, $completer:expr, $args:literal) => {
        register_slash_command! {_create_extern_command_wrapper $ident}
        register_slash_command!{_register_command $ident, $name, $completer, $args}
    };
}
pub(crate) use register_slash_command;