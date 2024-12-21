use pg_dig_server::{connect, start_replication};
use pg_dig_server::libpq::PQfinish;

const LOCAL_CONNECTION_STRING: &str = "host=localhost user=postgres dbname=postgres password=postgres replication=database";

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