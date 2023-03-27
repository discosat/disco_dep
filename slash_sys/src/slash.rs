use crate::slash_sys::{slash_command, slash};
use crate::{SlashError, SlashResult, SlashExitCode};

pub struct Slash<'a>(&'a slash);

impl<'a> Slash<'a> {
    pub unsafe fn from_raw(slash: *mut slash) -> SlashResult<Slash<'a>> {
        if let Some(slash_ref) = slash.as_ref() {
            Ok(Slash(slash_ref))
        } else {
            Err(SlashError::ExitCode(SlashExitCode::Exit))
        }
    }

    pub(crate) const fn inner(&self) -> &slash {
        self.0
    }
}

unsafe impl Sync for slash_command {}

/// Register a new command to slash.
///
/// This takes the same arguments as __slash_command in slash
#[macro_export]
macro_rules! register_slash_command {
    // Create an extern wrapper around a command
    (_create_extern_command_wrapper $name:ident) => {
        $crate::paste::paste!{
            #[no_mangle]
            pub unsafe extern "C" fn [<extern_$name>] (slash: *mut $crate::slash_sys::slash) -> std::os::raw::c_int {
                let slash = match $crate::Slash::from_raw(slash) {
                    Ok(slash) => slash,
                    Err(e) => {
                        return $crate::SlashExitCode::from(e).code();
                    }
                };

                let res: Result<_, $crate::SlashExitCode> = $name(&slash).map_err::<$crate::SlashExitCode, _>(From::from);
                
                if let Err(code) = res {
                    code.code()
                } else {
                    $crate::slash_sys::SLASH_SUCCESS as std::os::raw::c_int
                }
            }
        }
        
    };

    // Register the command with slash
    (_register_command $ident:ident, $name:literal, $completer:expr, $args:literal) => {
        $crate::paste::paste!{
            #[link_section = "slash"]
            #[used]
            pub static [<static_$ident>]: $crate::slash_sys::slash_command = $crate::slash_sys::slash_command {
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