use std::borrow::BorrowMut;
use std::ffi::{c_char, CString};
use std::marker::PhantomData;
use std::ptr::null_mut;
use std::slice::from_raw_parts;

use crate::slash_sys::{optparse_add_string, optparse_parse, optparse_del, optparse_add_double, optparse_add_int, optparse, optparse_new, optparse_add_help, optparse_add_unsigned};
use crate::{SlashResult, SlashError, Slash};

pub struct OptionParser<'a> {
    parser: *mut optparse, 
    unnamed_args: Vec<UnnamedArgument<'a>>,
    /// Store all string arguments so they can be converted to rust Strings when parsing
    string_args: Vec<(Box<*mut c_char>, &'a mut String)>,
    /// Keep all strings to stop them form being deallocated
    strings: Vec<CString>,
    lifetime: PhantomData<&'a ()>
}

pub struct UnnamedArgument<'a> {
    pub value: &'a mut String,
    pub missing_error: String
}

impl<'a> OptionParser<'a> {
    pub fn new<N: AsRef<str>, H: AsRef<str>>(progname: N, arg_summary: H) -> SlashResult<OptionParser<'a>> {
        unsafe {
            let c_progname = CString::new(progname.as_ref()).unwrap();
            let c_arg_summary = CString::new(arg_summary.as_ref()).unwrap();
            
            Ok(OptionParser {
                parser: optparse_new(
                    c_progname.as_ptr(),
                    c_arg_summary.as_ptr(),
                ),
                unnamed_args: Vec::new(),
                string_args: Vec::new(),
                strings: vec![c_progname, c_arg_summary],
                lifetime: PhantomData
            })
        }
    }

    pub fn add_help(self) -> Self {
        unsafe {
            optparse_add_help(self.parser);
        }
        self
    }

    pub fn add_unsigned(mut self, short: char, long: &str, desc: &str, base: u32, value: &'a mut u32, help: &str) -> SlashResult<Self> 
    where
        Self: 'a
    {
        let c_short = short as u32;
        if c_short > c_char::MAX as u32 {
            return Err(SlashError::InvalidCharacter { failed_on: short });
        }
        let c_long = CString::new(long).unwrap();
        let c_desc = CString::new(desc).unwrap();
        let c_help = CString::new(help).unwrap();

        unsafe {
            optparse_add_unsigned(
                self.parser, 
                c_short as i32, 
                c_long.as_ptr(), 
                c_desc.as_ptr(), 
                base, 
                value as *mut u32, 
                c_help.as_ptr() as *mut c_char
            );
        }

        self.strings.push(c_long);
        self.strings.push(c_desc);
        self.strings.push(c_help);

        Ok(self)
    }

    pub fn add_int(mut self, short: char, long: &str, desc: &str, base: u32, value: &'a mut i32, help: &str) -> SlashResult<Self> 
    where
        Self: 'a
    {
        let c_short = short as u32;
        if c_short > c_char::MAX as u32 {
            return Err(SlashError::InvalidCharacter { failed_on: short });
        }
        let c_long = CString::new(long).unwrap();
        let c_desc = CString::new(desc).unwrap();
        let c_help = CString::new(help).unwrap();

        unsafe {
            optparse_add_int(
                self.parser, 
                c_short as i32, 
                c_long.as_ptr(), 
                c_desc.as_ptr(), 
                base,
                value as *mut i32, 
                c_help.as_ptr() as *mut c_char
            );
        }

        self.strings.push(c_long);
        self.strings.push(c_desc);
        self.strings.push(c_help);

        Ok(self)
    }

    pub fn add_double(mut self, short: char, long: &str, desc: &str, value: &'a mut f64, help: &str) -> SlashResult<Self> 
    where
        Self: 'a
    {
        let c_short = short as u32;
        if c_short > c_char::MAX as u32 {
            return Err(SlashError::InvalidCharacter { failed_on: short });
        }
        let c_long = CString::new(long).unwrap();
        let c_desc = CString::new(desc).unwrap();
        let c_help = CString::new(help).unwrap();

        unsafe {
            optparse_add_double(
                self.parser, 
                c_short as i32, 
                c_long.as_ptr(), 
                c_desc.as_ptr(), 
                value as *mut f64, 
                c_help.as_ptr() as *mut c_char
            );
        }

        self.strings.push(c_long);
        self.strings.push(c_desc);
        self.strings.push(c_help);

        Ok(self)
    }
    
    pub fn add_string(mut self, short: char, long: &str, desc: &str, value: &'a mut String, help: &str) -> SlashResult<Self> 
    where
        Self: 'a
    {
        let c_short = short as u32;
        if c_short > c_char::MAX as u32 {
            return Err(SlashError::InvalidCharacter { failed_on: short });
        }
        let c_long = CString::new(long).unwrap();
        let c_desc = CString::new(desc).unwrap();
        let c_help = CString::new(help).unwrap();
        let mut c_value = Box::new(null_mut() as *mut c_char);

        unsafe {
            optparse_add_string(
                self.parser, 
                c_short as i32, 
                c_long.as_ptr(), 
                c_desc.as_ptr(), 
                c_value.borrow_mut(), 
                c_help.as_ptr() as *mut c_char
            );
        }

        self.strings.push(c_long);
        self.strings.push(c_desc);
        self.strings.push(c_help);

        self.string_args.push((c_value, value));

        Ok(self)
    }

    pub fn add_unnamed_string(mut self, value: &'a mut String, missing_error: String) -> Self {
        self.unnamed_args.push(UnnamedArgument { 
            value, 
            missing_error: missing_error 
        });

        self
    }

    pub fn parse(mut self, slash: &Slash) -> SlashResult<()> {

        extern "C" {
            /// Provided by libc or compiler_builtins.
            fn strlen(s: *const c_char) -> usize;
        }

        unsafe {
            let argi = optparse_parse(
                self.parser, 
                slash.inner().argc - 1, 
                slash.inner().argv.offset(1) as *mut *const c_char
            );

            // Check if the parser failed
            if argi < 0 {
                return Err(SlashError::ExitCode(crate::SlashExitCode::Einval));
            }

            // Handle unnamed arguments
            let iter = self.unnamed_args.iter_mut().enumerate();
            for (i, unnamed) in iter {
                if argi + i as i32 + 1 >= slash.inner().argc {
                    eprintln!("{}", unnamed.missing_error);
                    return Err(SlashError::ExitCode(crate::SlashExitCode::Einval));
                }

                let str_ptr = *slash.inner().argv.offset(argi as isize + i as isize + 1);
                let len = strlen(str_ptr);
                let chars = from_raw_parts(str_ptr, len);

                // NOTE: the value that is returned, 
                // is a copy. Allowing us to free the string when the parser goes out of scope
                *unnamed.value = chars.into_iter().map(|c| *c as u8 as char).collect();
            }

            // Handle named string arguments
            for (c_value, value) in &mut self.string_args {
                let str_ptr = **c_value;
                
                if !str_ptr.is_null() {
                    let len = strlen(str_ptr);
                    let chars = from_raw_parts(str_ptr, len);

                    // NOTE: the value that is returned, 
                    // is a copy. Allowing us to free the string when the parser goes out of scope
                    **value = chars.into_iter().map(|c| *c as u8 as char).collect();
                }
            }
        }

        Ok(())
    }
}

impl<'a> Drop for OptionParser<'a> {
    fn drop(&mut self) {
        unsafe {
            // Free all dependencies of the parser
            optparse_del(self.parser);
        }
    }
}