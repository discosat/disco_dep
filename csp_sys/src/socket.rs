use crate::{csp_sys::{csp_socket_t, csp_bind, csp_listen, csp_accept}, CSPResult, CSPError, Connection};

pub struct Socket(csp_socket_t);


impl Socket {
    pub fn new() -> Socket {
        Socket(csp_socket_t {
            rx_queue: std::ptr::null_mut(),
            rx_queue_static: std::ptr::null_mut(),
            rx_queue_static_data: [0; 8000usize],
            opts: 0,
        })        
    }

    pub fn bind(&mut self, port: u8) -> CSPResult<()> {
        unsafe {
            let err = csp_bind(&mut self.0, port);
            CSPError::from_int(err)?;
            
            Ok(())
        }
    }

    pub fn listen(&mut self, backlog: usize) -> CSPResult<()> {
        unsafe {
            let err = csp_listen(&mut self.0, backlog);
            CSPError::from_int(err)?;

            Ok(())
        }
    }

    pub fn accept(&mut self, timeout: u32) -> Option<Connection> {
        unsafe {
            let conn = csp_accept(&mut self.0, timeout);

            if conn.is_null() {
                None
            } else {
                Some(Connection(conn))
            }
        }
    }
}
