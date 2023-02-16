use crate::csp_sys::csp_conn_print_table;

// Here we expose it so we can call it from csh
#[no_mangle]
pub extern "C" fn rust_csp_conn_print_table() {

    println!("rust_csp_conn_print_table() is called");
    unsafe {
        csp_conn_print_table();
    }
}