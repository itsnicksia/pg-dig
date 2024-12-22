#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod lsn;

pub mod libpq {
    use crate::lsn::Lsn;
    use chrono::{DateTime, Days, Months};
    use libc::{free, malloc};
    use scroll::Pread;
    use std::ffi::{c_char, c_int, c_void, CStr, CString};
    use std::slice;

    include!(concat!(env!("OUT_DIR"), "/libpq_bindings.rs"));

    #[repr(C)]
    #[derive(Debug, Pread)]
    struct TransactionId(u32);

    #[repr(C)]
    #[derive(Debug, Pread)]
    struct RmgrId(u8);

    #[repr(C)]
    #[derive(Debug, Pread)]
    struct XLogRecord {
        xl_tot_len: u32,            /* total len of entire record */
        xl_xid: TransactionId,      /* xact id */
        xl_prev: u64,        /* ptr to previous record in log */
        xl_info: u8,                /* flag bits, see below */
        xl_rmid: RmgrId,            /* resource manager for this record */
        padding: [u8;2],            /* 2 bytes of padding here, initialize to zero */
        xl_crc: u32,                /* CRC for this record */
        /* XLogRecordBlockHeaders and XLogRecordDataHeader follow, no padding */
    }

    /*
        dataStart = pq_getmsgint64(&incoming_message);
        walEnd = pq_getmsgint64(&incoming_message);
        sendTime = pq_getmsgint64(&incoming_message);
        ProcessWalSndrMessage(walEnd, sendTime);
     */
    #[repr(C)]
    #[derive(Debug, Pread)]
    struct XLogHeader {
        dataStart: u64,
        walEnd: u64,
        sendTime: u64,
    }

    pub unsafe fn print_status(conn: *mut PGconn) {
        let conn_status = friendly_conn_status(PQstatus(conn));

        let result = PQgetResult(conn);
        let raw_error_message = CStr::from_ptr(PQresultErrorMessage(result)).to_str().expect("failed to parse error message");
        let error_message = if raw_error_message.is_empty() { "none" } else { raw_error_message };

        let result_status = PQresultStatus(result);

        println!("{} | result: {} | error: {}", conn_status, friendly_exec_status(result_status), error_message);
    }

    pub unsafe fn to_cstr(string: &str) -> CString {
        match CString::new(string) {
            Ok(cstr) => cstr,
            Err(_) => todo!(),
        }
    }

    pub unsafe fn exec(conn: *mut PGconn, stmt: &str) -> *mut PGresult {
        println!("exec: {}", stmt);
        let result = PQexec(conn, to_cstr(stmt).as_ptr());
        print_status(conn);
        result
    }


    pub unsafe fn connect(conn_string: &str) -> *mut pg_conn {
        let conn_string = to_cstr(conn_string);
        println!("connecting: {}", conn_string.to_str().expect("failed to parse connection string"));
        let conn = PQconnectdb(conn_string.as_ptr());
        print_status(conn);
        conn
    }

    pub unsafe fn start_replication(conn: *mut PGconn) -> Result<(), String> {
        let result = exec(conn,"START_REPLICATION SLOT physical PHYSICAL 0/1542480");
        match PQresultStatus(result) {
            ExecStatusType_PGRES_COPY_BOTH => Ok(()),
            other => Err(friendly_exec_status(other)),
        }
    }

    pub unsafe fn start_replicating(conn: *mut PGconn, consumer: fn(String)) -> Result<i32, String> {
        let mut buffer: *mut c_char = malloc(1024) as *mut c_char;
        println!("buffer@{}", buffer as isize);

        loop {
            if PQconsumeInput(conn) == 0 {
                println!("error!");
                return Err(String::from("failed to consume input"))
            }
            let length: Result<c_int, String> = match PQgetCopyData(conn, &mut buffer, 0) {
                length if length > size_of::<XLogRecord>().try_into().expect("XLogRecord struct is too large") => Ok(length),
                -1 => Err("replication: done!".to_string()),
                -2 => Err("replication: copy failed!".to_string()),
                _ => panic!("not yet implemented")
            };

            if length.is_err() {
                break;
            }

            println!("[message bytes]");
            for byte in slice::from_raw_parts(buffer as *const u8, size_of::<XLogHeader>() + size_of::<XLogRecord>()) {
                print!("{:02x} ", byte);
            }
            println!();

            match *buffer as u8 as char {
                'w' => {
                    processWalRecordHeader(buffer.add(1));
                    processWalRecord(buffer.add(25), consumer)
                },
                other => panic!("unexpected value: {}", other)
            }
        }
        free(buffer as *mut c_void);
        Ok(0)
    }

    unsafe fn processWalRecordHeader(buffer: *mut c_char) {
        let header_slice = slice::from_raw_parts(buffer as *const u8, size_of::<XLogHeader>());

        println!("[header bytes]");
        for byte in header_slice {
            print!("{:02x} ", byte);
        }

        let xlog_header = header_slice
            .pread_with::<XLogHeader>(0, scroll::BE)
            .expect("failed to read xlog record header");

        println!("dataStart: {}", Lsn::from_u64(xlog_header.dataStart));
        println!("walEnd: {}", Lsn::from_u64(xlog_header.walEnd));
        println!("sendTime: {}",
             DateTime::from_timestamp_micros(xlog_header.sendTime.try_into().expect("sendTime too large"))
            .expect("error")
                 .checked_add_months(Months::new(360)).unwrap()
                 .checked_sub_days(Days::new(1)).unwrap()
        );
    }

    unsafe fn processWalRecord(buffer: *mut c_char, consumer: fn(String)) {
        let record_slice = slice::from_raw_parts(buffer as *const u8, size_of::<XLogRecord>());

        println!("[record bytes]");
        for byte in
            record_slice {
            print!("{:02x} ", byte);
        }
        println!();

        let xlog_record = record_slice
            .pread_with::<XLogRecord>(0, scroll::LE)
            .expect("failed to read xlog record");

        println!("xlog record: {:#?}", xlog_record);
        consumer(String::from("poop"));
    }


    pub unsafe fn friendly_exec_status(exec_status_type: ExecStatusType) -> String {
        String::from(match exec_status_type {
            ExecStatusType_PGRES_EMPTY_QUERY => "PGRES_EMPTY_QUERY",
            ExecStatusType_PGRES_COMMAND_OK => "PGRES_COMMAND_OK",
            ExecStatusType_PGRES_TUPLES_OK => "PGRES_TUPLES_OK",
            ExecStatusType_PGRES_COPY_OUT => "PGRES_COPY_OUT",
            ExecStatusType_PGRES_COPY_IN => "PGRES_COPY_IN",
            ExecStatusType_PGRES_BAD_RESPONSE => "PGRES_BAD_RESPONSE",
            ExecStatusType_PGRES_NONFATAL_ERROR => "PGRES_NONFATAL_ERROR",
            ExecStatusType_PGRES_FATAL_ERROR => "PGRES_FATAL_ERROR",
            ExecStatusType_PGRES_COPY_BOTH => "PGRES_COPY_BOTH",
            ExecStatusType_PGRES_SINGLE_TUPLE => "PGRES_SINGLE_TUPLE",
            ExecStatusType_PGRES_PIPELINE_SYNC => "PGRES_PIPELINE_SYNC",
            ExecStatusType_PGRES_PIPELINE_ABORTED => "PGRES_PIPELINE_ABORTED",
            ExecStatusType_PGRES_TUPLES_CHUNK => "PGRES_TUPLES_CHUNK",
            other => panic!("invalid exec status: {}", other),
        })
    }

    pub unsafe fn friendly_conn_status(conn_status_type: ConnStatusType) -> String {
        String::from(match conn_status_type {
            ConnStatusType_CONNECTION_OK => "connection ok",
            ConnStatusType_CONNECTION_BAD => "connection bad",
            ConnStatusType_CONNECTION_STARTED => "connection started",
            ConnStatusType_CONNECTION_MADE => "connection made",
            ConnStatusType_CONNECTION_AWAITING_RESPONSE => "connection awaiting response",
            ConnStatusType_CONNECTION_AUTH_OK => "connection auth ok",
            ConnStatusType_CONNECTION_SETENV => "connection setenv",
            ConnStatusType_CONNECTION_SSL_STARTUP => "connection ssl startup",
            ConnStatusType_CONNECTION_NEEDED => "connection needed",
            ConnStatusType_CONNECTION_CHECK_WRITABLE => "connection check writable",
            ConnStatusType_CONNECTION_CONSUME => "connection consume",
            ConnStatusType_CONNECTION_GSS_STARTUP => "connection gss startup",
            ConnStatusType_CONNECTION_CHECK_TARGET => "connection check target",
            ConnStatusType_CONNECTION_CHECK_STANDBY => "connection check standby",
            ConnStatusType_CONNECTION_ALLOCATED => "connection allocated",
            other => panic!("invalid connection status: {}", other),
        })
    }
}
