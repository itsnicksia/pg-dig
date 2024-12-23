#![allow(unused_variables)]
#![allow(unsafe_code)]

use pg_dig_server::postgres::bindings::PQfinish;
use pg_dig_server::postgres::replication::{connect, start_replicating, start_replication};

const LOCAL_CONNECTION_STRING: &str = "host=localhost user=postgres dbname=postgres password=postgres replication=database";
fn main() {
    unsafe {
        let conn = connect(LOCAL_CONNECTION_STRING);
        start_replication(conn).unwrap();
        start_replicating(conn, |s: String| { println!()}).unwrap();
        PQfinish(conn);
    }
}
