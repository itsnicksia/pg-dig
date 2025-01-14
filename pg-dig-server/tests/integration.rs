#![allow(dead_code)]

use pg_dig_server::postgres::replication::{start_replication};
use pg_dig_server::postgres::bindings::{PQfinish};
use pg_dig_server::postgres::connection::connect;

const LOCAL_CONNECTION_STRING: &str = "host=localhost user=postgres dbname=postgres password=postgres replication=database";
const TEST_BUFFER: [u8; 1024] = [0x77, 0x0, 0x0, 0x0, 0x0, 0x01, 0x55, 0x2c, 0x80, 0x0, 0x0, 0x0, 0x0, 0x01, 0x55, 0xe0, 0x80, 0x0, 0x02, 0xcc, 0xe9, 0xaa, 0x79, 0xd4, 0x0, 0x2a, 0x0, 0x0, 0x0, 0xea, 0x02, 0x0, 0x0, 0x48, 0x2c, 0x55, 0x01, 0x0, 0x0, 0x0, 0x0, 0x0, 0x08, 0x0, 0x0, 0x06, 0x06, 0x87, 0x20, 0xff, 0x10, 0x01, 0x0, 0x0, 0x0, 0xea, 0x02, 0x0, 0x0, 0x05, 0x0, 0x0, 0x0, 0x01, 0x60, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1e, 0x0, 0x0, 0x0, 0xea, 0x02, 0x0, 0x0, 0x80, 0x2c, 0x55, 0x01, 0x0, 0x0, 0x0, 0x0, 0x30, 0x0, 0x0, 0x0, 0xac, 0x5b, 0x64, 0xc0, 0xff, 0x04, 0x0, 0xa0, 0x0, 0x0, 0x0, 0x0, 0x2a, 0x0, 0x0, 0x0, 0xea, 0x02, 0x0, 0x0, 0xb0, 0x2c, 0x55, 0x01, 0x0, 0x0, 0x0, 0x0, 0x11, 0x02, 0x0, 0x0, 0x91, 0xd2, 0xe9, 0x74, 0xff, 0x10, 0x7f, 0x06, 0x0, 0x0, 0x05, 0x0, 0x0, 0x0, 0x0, 0x80, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3c, 0x0, 0x0, 0x0, 0xea, 0x02, 0x0, 0x0, 0xd0, 0x2c, 0x55, 0x01, 0x0, 0x0, 0x0, 0x0, 0x70, 0x09, 0x0, 0x0, 0xe0, 0x83, 0xa8, 0xce, 0xff, 0x22, 0xea, 0x02, 0x0, 0x0, 0xff, 0xff, 0xff, 0xff, 0x0, 0x0, 0x0, 0x0, 0xff, 0xff, 0xff, 0xff, 0x7f, 0x06, 0x0, 0x0, 0x05, 0x0, 0x0, 0x0, 0xeb, 0x04, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x03, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3c, 0x0, 0x0, 0x0, 0xea, 0x02, 0x0, 0x0, 0x0, 0x2d, 0x55, 0x01, 0x0, 0x0, 0x0, 0x0, 0x70, 0x09, 0x0, 0x0, 0x6b, 0x2c, 0x0a, 0x36, 0xff, 0x22, 0xea, 0x02, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, 0x06, 0x0, 0x0, 0x05, 0x0, 0x0, 0x0, 0xeb, 0x04, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x06, 0x0, 0x0, 0x0, 0x0, 0x0, 0x45, 0x06, 0x0, 0x0, 0xea, 0x02, 0x0, 0x0, 0x40, 0x2d, 0x55, 0x01, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0a, 0x0, 0x0, 0x83, 0x2d, 0xeb, 0xec, 0x0, 0x10, 0x0, 0x0, 0x04, 0x06, 0xd4, 0x0, 0x03, 0x7f, 0x06, 0x0, 0x0, 0x05, 0x0, 0x0, 0x0, 0xeb, 0x04, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xff, 0x0e, 0x0, 0x0, 0x0, 0x0, 0x38, 0x16, 0x55, 0x01, 0x0, 0x0, 0x01, 0x0, 0xd4, 0x0, 0xd0, 0x1a, 0x0, 0x20, 0x04, 0x20, 0xea, 0x02, 0x0, 0x0, 0x2f, 0x0, 0x01, 0x0, 0x90, 0x9d, 0x58, 0x01, 0xe0, 0x9c, 0x58, 0x01, 0x30, 0x9c, 0x58, 0x01, 0x80, 0x9b, 0x58, 0x01, 0xd0, 0x9a, 0x58, 0x01, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2e, 0x0, 0x01, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x28, 0x9f, 0xa2, 0x01, 0x40, 0x9e, 0xc2, 0x01, 0xea, 0x02, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x06, 0x0, 0x21, 0x0, 0x01, 0x28, 0x20, 0xff, 0xff, 0xff, 0x3f, 0x0, 0x0, 0x0, 0x0, 0x0, 0x01, 0x60, 0x0, 0x0, 0x65, 0x6d, 0x70, 0x6c, 0x6f, 0x79, 0x65, 0x65, 0x73, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x98, 0x08, 0x0, 0x0, 0x03, 0x60, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0a, 0x0, 0x0, 0x0, 0x02, 0x0, 0x0, 0x0, 0x0, 0x80, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x80, 0xbf, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x01, 0x0, 0x70, 0x72, 0x09, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x01, 0x64, 0x0, 0x0, 0x0, 0x0, 0x0, 0xea, 0x02, 0x0, 0x0, 0x01, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xe6, 0x02, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x06, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x05, 0x0, 0x21, 0x0, 0x01, 0x09, 0x20, 0xff, 0xff, 0xff, 0x3f, 0x0, 0x0, 0x0, 0x0, 0x0, 0x07, 0x60, 0x0, 0x0, 0x65, 0x6d, 0x70, 0x6c, 0x6f, 0x79, 0x65, 0x65, 0x73, 0x5f, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x5f, 0x6b, 0x65, 0x79, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x98, 0x08, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0a, 0x0, 0x0, 0x0, 0x93, 0x01, 0x0, 0x0, 0x07, 0x60, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x01, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x70, 0x69, 0x01, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x01, 0x6e, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xe6, 0x02, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x04, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x04, 0x0, 0x21, 0x0, 0x01, 0x09, 0x20, 0xff, 0xff, 0xff, 0x3f, 0x0, 0x0, 0x0, 0x0, 0x0, 0x05, 0x60, 0x0, 0x0, 0x65, 0x6d, 0x70, 0x6c, 0x6f, 0x79, 0x65, 0x65, 0x73, 0x5f, 0x70, 0x6b, 0x65, 0x79, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x98, 0x08, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0a, 0x0, 0x0, 0x0, 0x93, 0x01, 0x0, 0x0, 0x05, 0x60, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];

#[test]
fn test_replication() {
    unsafe {
        let conn = connect(LOCAL_CONNECTION_STRING);
        let result = start_replication(conn);

        assert!(
            result.is_ok(),
            "Expected Ok(()), got Err({:?})",
            result.err());

        PQfinish(conn);
    }
}