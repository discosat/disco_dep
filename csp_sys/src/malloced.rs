use crate::malloc_sys::free;
use std::ops::Deref;
use std::os::raw::c_void;
use std::fmt;
use std::fmt::Debug;
use crate::{CSPError, CSPResult};

pub struct Malloced<T>(*mut T)
where
    T: Sized + Sync + Send;

impl<T> Malloced<T>
where
    T: Sized + Sync + Send
{
    pub unsafe fn from_raw<P>(ptr: *mut P) -> CSPResult<Malloced<T>> 
    where
        P: Sized + Sync + Send
    {
        if ptr.is_null() {
            return Err(CSPError::NullPointerCast);
        }

        Ok(Malloced(ptr as *mut T))
    }
}

impl<T> Drop for Malloced<T>
where
    T: Sized + Sync + Send
{
    fn drop(&mut self) {
        unsafe {
            free(self.0 as *mut c_void);
        }
    }
}

impl<T> Deref for Malloced<T>
where
    T: Sized + Sync + Send
{
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            self.0.as_ref().unwrap()
        }
    }
}

impl<T> Debug for Malloced<T>
where
    T: Sized + Sync + Send + Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Malloced")
         .field(self.deref())
         .finish()
    }
}