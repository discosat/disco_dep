use crate::register_slash_command;

/// Force everything here to be loaded by linker
/// This works because it is being used in the header file
/// For files with other extern functions this might not be necessary
#[no_mangle]
pub static RUST_SLASH_LOAD: u32 = 0;

register_slash_command!{helloa, "Helloa", None, None, ""}
register_slash_command!{hellob, "Hellob", None, None, ""}
register_slash_command!{helloc, "Helloc", None, None, ""}