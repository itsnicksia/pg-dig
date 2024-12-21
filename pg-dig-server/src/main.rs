#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(unsafe_code)]

use pg_dig_server::libpq::*;
use pg_dig_server::{connect, start_replicating, start_replication};

const LOCAL_CONNECTION_STRING: &str = "host=localhost user=postgres dbname=postgres password=postgres replication=database";
fn main() {
    unsafe {
        let conn = connect(LOCAL_CONNECTION_STRING);
        start_replication(conn).unwrap();
        start_replicating(conn).unwrap();
        PQfinish(conn);
    }
}
