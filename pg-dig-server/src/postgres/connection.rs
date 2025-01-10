use std::ffi::CString;
use crate::postgres::bindings::{pg_conn, PQconnectdb};
use crate::postgres::pg_conn::print_status;

pub unsafe fn connect(conn_string: &str) -> *mut pg_conn {
    let conn_string = CString::new(conn_string).expect("failed to build connection string");
    println!("connecting: {:?}", conn_string);
    let conn = PQconnectdb(conn_string.as_ptr());
    print_status(conn);
    conn
}