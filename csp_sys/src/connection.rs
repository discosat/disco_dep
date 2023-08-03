use crate::{CSPResult, CSPError, Packet, MallocedSlice};
use crate::csp_sys::{csp_conn_t, csp_sfp_recv_fp, csp_close, csp_connect, csp_prio_t, csp_conn_dport, csp_conn_sport, csp_read, csp_send, csp_sfp_send_own_memcpy, csp_prio_t_CSP_PRIO_CRITICAL, csp_prio_t_CSP_PRIO_HIGH, csp_prio_t_CSP_PRIO_NORM, csp_prio_t_CSP_PRIO_LOW};
use crate::Malloced;

pub struct Connection(pub(crate) *mut csp_conn_t);
use std::mem::size_of;
use std::os::raw::{c_uint, c_int, c_ulong, c_void};
use std::ptr::null_mut;

#[derive(Clone, Copy)]
pub enum ConnectionOption {
    RDP = 0x0001,
    NORDP = 0x0002,
    HMAC = 0x0004,
    NOHMAC = 0x0008,
    CRC32 = 0x0040,
    NOCRC32 = 0x0080,
    SAME = 0x8000
}

#[repr(u8)]
pub enum ConnectionPriority {
    Critical = csp_prio_t_CSP_PRIO_CRITICAL as u8,
    High = csp_prio_t_CSP_PRIO_HIGH as u8,
    Norm = csp_prio_t_CSP_PRIO_NORM as u8,
    Low = csp_prio_t_CSP_PRIO_LOW as u8
}
pub type NodeId = u16;

impl Connection {
    pub fn connect(prio: ConnectionPriority, dst: NodeId, dst_port: u8, timeout: u32, options: &[ConnectionOption]) -> CSPResult<Connection> {
        unsafe {
            let opts = options
                .iter()
                .fold(0, |f, opt| f | *opt as u32);

            let conn = csp_connect(csp_prio_t_CSP_PRIO_HIGH as u8, dst, dst_port, timeout, opts);

            if conn.is_null() {
                return Err(CSPError::ConnectionFailed)
            } else {
                return Ok(Connection(conn));
            }
        }
    }

    pub fn read<T>(&mut self, timeout: u32) -> CSPResult<Packet<T>> 
    where
        T: Sized + Copy
    {
        unsafe {
            let packet = csp_read(self.0, timeout);

            if packet.is_null() {
                Err(CSPError::ReadFailed)
            } else {
                Packet::<T>::from_raw(packet)
            }
        }
    }

    pub fn send<T>(&mut self, payload: &T) -> CSPResult<()>
    where
        T: Sized + Copy
    {
        unsafe {
            let packet = Packet::new(payload)?;
            csp_send(self.0, packet.0);
        }

        Ok(())
    }

    pub fn send_sfp<T>(&mut self, content: &T, mtu: u32, timout: u32) -> CSPResult<()> 
    where
        T: Sized + Copy
    {
        unsafe {
            let res = csp_sfp_send(
                self.0, content as *const T as *const c_void, 
                size_of::<T>() as u32, 
                mtu, 
                timout
            );
            CSPError::from_int(res)?;
        }

        Ok(())
    }

    pub fn send_sfp_slice<T>(&mut self, content: &[T], mtu: u32, timout: u32) -> CSPResult<()> 
    where
        T: Sized + Send
    {
        unsafe {
            let res = csp_sfp_send(
                self.0, content.as_ptr() as *const c_void, 
                (size_of::<T>() * content.len()) as u32, 
                mtu, 
                timout
            );
            CSPError::from_int(res)?;
        }

        Ok(())
    }

    pub fn read_sfp<T>(&mut self, timeout: u32) -> CSPResult<Malloced<T>>
    where
        T: Sized + Send
    {
        let mut data = null_mut();
        let mut data_size: c_int = 0;

        unsafe {
            let res = csp_sfp_recv(self.0, &mut data, &mut data_size, timeout);
            CSPError::from_int(res)?;

            if (data_size as usize) < size_of::<T>() {
                Err(CSPError::SFPConvertionError)
            } else {
                Ok(Malloced::from_raw(data as *mut T)?)
            }
        }
    }

    pub fn read_sfp_slice<T>(&mut self, timeout: u32) -> CSPResult<MallocedSlice<T>> 
    where
        T: Sized + Send
    {
        let mut data = null_mut();
        let mut data_size: c_int = 0;

        unsafe {
            let res = csp_sfp_recv(self.0, &mut data, &mut data_size, timeout);
            CSPError::from_int(res)?;

            let value_size = size_of::<T>();

            if data_size as usize % size_of::<T>() != 0 {
                return Err(CSPError::SFPConvertionError);
            }

            MallocedSlice::from_raw_parts(
                data as *mut T, 
                data_size as usize / value_size
            )
        }
    }

    pub fn destination_port(&self) -> u8 {
        unsafe {
            csp_conn_dport(self.0) as u8
        }
    }

    pub fn source_port(&self) -> u8 {
        unsafe {
            csp_conn_sport(self.0) as u8
        }
    }
}

pub unsafe extern "C" fn memcpy_conn(
    __dest: *mut ::std::os::raw::c_void,
    __src: *const ::std::os::raw::c_void,
    __n: usize,
) -> *mut ::std::os::raw::c_void {
    crate::csp_sys::memcpy(__dest, __src, __n as c_ulong)
}

unsafe fn csp_sfp_send(conn: *mut csp_conn_t, data: *const c_void, datasize: c_uint, mtu: c_uint, timeout: u32) -> c_int {
    return csp_sfp_send_own_memcpy(conn, data, datasize, mtu, timeout, Some(memcpy_conn));
}

unsafe fn csp_sfp_recv(conn: *mut csp_conn_t, dataout: *mut *mut c_void, datasize: *mut c_int, timeout: u32) -> c_int {
    return csp_sfp_recv_fp(conn, dataout, datasize, timeout, null_mut());
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            csp_close(self.0);
        }
    }
}
/*
pub fn csp_send(conn: *mut csp_conn_t, packet: *mut csp_packet_t);
pub fn csp_read(conn: *mut csp_conn_t, timeout: u32) -> *mut csp_packet_t;
static inline int csp_sfp_recv(csp_conn_t * conn, void ** dataout, int * datasize, uint32_t timeout) {
static inline int csp_sfp_send(csp_conn_t * conn, const void * data, unsigned int datasize, unsigned int mtu, uint32_t timeout) {
*/