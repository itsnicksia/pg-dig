#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::ffi::{c_char, c_int, CStr, CString};
use std::slice;
use chrono::{DateTime, Days, Months};
use scroll::Pread;
use crate::postgres::bindings::*;
use crate::postgres::lsn::Lsn;
use crate::postgres::types::{XLogMessageHeader, XLogRecordHeader};

pub unsafe fn print_status(conn: *mut PGconn) {
    let conn_status = friendly_conn_status(PQstatus(conn));

    let result = PQgetResult(conn);
    let raw_error_message = CStr::from_ptr(PQresultErrorMessage(result)).to_str().expect("failed to parse error message");
    let error_message = if raw_error_message.is_empty() { "none" } else { raw_error_message };

    let result_status = PQresultStatus(result);

    println!("{} | result: {} | error: {}", conn_status, friendly_exec_status(result_status), error_message);
}

pub unsafe fn exec(conn: *mut PGconn, stmt: &str) -> *mut PGresult {
    println!("exec: {}", stmt);
    let statement = CString::new(stmt).expect("failed to build statement");
    let result = PQexec(conn, statement.as_ptr());
    print_status(conn);
    result
}


pub unsafe fn connect(conn_string: &str) -> *mut pg_conn {
    let conn_string = CString::new(conn_string).expect("failed to build connection string");
    println!("connecting: {:?}", conn_string);
    let conn = PQconnectdb(conn_string.as_ptr());
    print_status(conn);
    conn
}

pub unsafe fn start_replication(conn: *mut PGconn) -> Result<(), String> {
    let result = exec(conn,"START_REPLICATION SLOT physical PHYSICAL 0/1552C80");
    match PQresultStatus(result) {
        ExecStatusType_PGRES_COPY_BOTH => Ok(()),
        other => Err(friendly_exec_status(other)),
    }
}

pub unsafe fn start_replicating(conn: *mut PGconn, consumer: fn(String)) -> Result<i32, String> {
    let mut buffer: *mut c_char = vec![0; 1024].as_mut_ptr();
    println!("buffer@{}", buffer as isize);

    loop {
        if PQconsumeInput(conn) == 0 {
            println!("error!");
            return Err(String::from("failed to consume input"))
        }
        let length: Result<c_int, String> = match PQgetCopyData(conn, &mut buffer, 0) {
            length if length > 0 => Ok(length),
            -1 => Err("replication: done!".to_string()),
            -2 => Err("replication: copy failed!".to_string()),
            other => panic!("not yet implemented: {}", other)
        };

        if length.is_err() {
            break;
        }

        match *buffer as u8 as char {
            'w' => processWalMessage(&mut *buffer, consumer),
            'k' => println!("*keep-alive received*"),
            other => panic!("unexpected record type: {}", other)
        }
    }
    Ok(0)
}

unsafe fn processWalMessage(buffer: *const c_char, consumer: fn(String)) {
    let u8_buffer: *const u8 = buffer as *const u8;
    let mut offset = 1;
    processWalRecordHeader(u8_buffer.add(offset));
    offset += size_of::<XLogMessageHeader>();

    processWalRecord(u8_buffer.add(offset), consumer);
    //offset += size_of::<XLogRecord>();
}

unsafe fn processWalRecordHeader(buffer: *const u8) {
    let header_slice = slice::from_raw_parts(buffer, size_of::<XLogMessageHeader>());

    let xlog_header = header_slice
        .pread_with::<XLogMessageHeader>(0, scroll::BE)
        .expect("failed to read xlog record header");

    println!("dataStart: {}", Lsn::from_u64(xlog_header.data_start));
    println!("walEnd: {}", Lsn::from_u64(xlog_header.wal_end));
    println!("sendTime: {}",
             DateTime::from_timestamp_micros(xlog_header.send_time.try_into().expect("sendTime too large"))
                 .expect("error")
                 /* FIXME: Not sure why it's off by exactly 30 years...? */
                 .checked_add_months(Months::new(360)).unwrap()
                 .checked_sub_days(Days::new(1)).unwrap()
    );
}

unsafe fn processWalRecord(buffer: *const u8, consumer: fn(String)) {
    let xlog_record = XLogRecordHeader::from_bytes(buffer);

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
