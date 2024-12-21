#![allow(unused_variables)]
#![allow(non_upper_case_globals)]

use pg_dig_server::libpq::*;
use pg_dig_server::{exec, print_status, to_cstr};

const LOCAL_CONNECTION_STRING: &str = "host=localhost user=postgres dbname=postgres password=postgres replication=database";
fn main() {
    unsafe {
        let conn = connect(LOCAL_CONNECTION_STRING);
        let result = exec(conn,"START_REPLICATION SLOT physical PHYSICAL 0/0");
        PQfinish(conn);
    }
}

unsafe fn connect(conn_string: &str) -> *mut pg_conn {
    let conn_string = to_cstr(LOCAL_CONNECTION_STRING);
    println!("connecting: {}", conn_string.to_str().unwrap());
    let conn = PQconnectdb(conn_string.as_ptr());
    print_status(conn);

    conn
}

