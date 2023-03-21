use std::ffi::c_void;
use std::marker::PhantomData;
use std::mem::{size_of, transmute};
use std::ops::Deref;
use std::fmt::Debug;
use std::fmt;
use crate::{CSPResult, CSPError};

use crate::csp_sys::{csp_packet_t, csp_buffer_get, CSP_BUFFER_SIZE, csp_buffer_free};

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
    )
}

pub struct Packet<T>(pub(crate) *mut csp_packet_t, PhantomData<T>)
where
    T: Sized + Sync + Send;

impl<T> Packet<T> 
where
    T: Sized + Sync + Send
{
    pub fn new(content: &T) -> CSPResult<Packet<T>> {
        unsafe {
            let mut packet = csp_buffer_get(size_of::<T>())
                .as_mut()
                .ok_or_else(|| CSPError::FailedToAllocBuffer)?;

            packet.length = size_of::<T>() as u16;

            let byte_content = any_as_u8_slice(content);
            for i in 0..CSP_BUFFER_SIZE {
                let idx = i as usize;

                if idx < byte_content.len() {
                    packet.__bindgen_anon_2.data[idx] = byte_content[idx];
                } else {
                    packet.__bindgen_anon_2.data[idx] = 0
                }
            }

            Ok(Packet(packet, PhantomData))
        }
    }

    pub unsafe fn from_raw(packet: *mut csp_packet_t) -> CSPResult<Packet<T>> {
        if packet.is_null() {
            return Err(CSPError::NullPointerCast);
        }

        if (*packet).length as usize != size_of::<T>() {
            return Err(CSPError::FailedPointerCast);
        }

        Ok(Packet(packet, PhantomData))
    }
}

impl<T> Packet<T> 
where
    T: Sized + Sync + Send + Clone
{
    pub fn unpack(self) -> T {
        self.deref().clone()
    }
}

impl<T> Drop for Packet<T>
where
    T: Sized + Sync + Send
{
    fn drop(&mut self) {
        unsafe {
            csp_buffer_free(self.0 as *mut c_void);
        }
    }
}

impl<T> Deref for Packet<T>
where
    T: Sized + Sync + Send
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            transmute(&(*self.0).__bindgen_anon_2.data)
        }
    }
}

impl<T> Debug for Packet<T>
where
    T: Sized + Sync + Send + Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Packet")
         .field(self.deref())
         .finish()
    }
}