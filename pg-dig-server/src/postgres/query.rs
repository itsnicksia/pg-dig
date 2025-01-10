use std::ffi::CString;
use crate::postgres::bindings::{PGconn, PGresult, PQexec};
use crate::postgres::pg_conn::print_status;

pub unsafe fn exec(conn: *mut PGconn, stmt: &str) -> *mut PGresult {
    println!("exec: {}", stmt);
    let statement = CString::new(stmt).expect("failed to build statement");
    let result = PQexec(conn, statement.as_ptr());
    print_status(conn);
    result
}