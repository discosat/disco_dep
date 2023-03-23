use std::ops::Deref;
use crate::{Malloced, CSPResult};
use std::slice::from_raw_parts;

pub struct MallocedSlice<'a, T>(pub Malloced<T>, pub &'a [T])
where
    T: Sized;


impl<'a, T> MallocedSlice<'a, T>
where
    T: Sized
{
    pub unsafe fn from_raw_parts(ptr: *mut T, n: usize) -> CSPResult<MallocedSlice<'a, T>> {
        let slice = from_raw_parts(ptr as *mut T, n as usize);

        Ok(MallocedSlice(Malloced::from_raw(ptr as *mut T)?, slice))
    }
}

impl<'a, T> Deref for MallocedSlice<'a, T>
where
    T: Sized
{
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.1
    }
}
