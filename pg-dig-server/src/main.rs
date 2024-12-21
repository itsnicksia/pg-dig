#![allow(unused_variables)]
#![allow(non_upper_case_globals)]

use pg_dig_server::libpq::*;
use pg_dig_server::print_status;
use std::ffi::CString;

fn main() {

    unsafe {
        let connection_str = CString::new("host=localhost user=postgres dbname=postgres password=postgres").unwrap();
        println!("{}", connection_str.to_str().unwrap());
        let conn = PQconnectdb(connection_str.as_ptr());
        print_status(conn);
    }
}