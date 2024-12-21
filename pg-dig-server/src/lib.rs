#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


use crate::libpq::{pg_conn, ConnStatusType, ConnStatusType_CONNECTION_ALLOCATED, ConnStatusType_CONNECTION_AUTH_OK, ConnStatusType_CONNECTION_AWAITING_RESPONSE, ConnStatusType_CONNECTION_BAD, ConnStatusType_CONNECTION_CHECK_STANDBY, ConnStatusType_CONNECTION_CHECK_TARGET, ConnStatusType_CONNECTION_CHECK_WRITABLE, ConnStatusType_CONNECTION_CONSUME, ConnStatusType_CONNECTION_GSS_STARTUP, ConnStatusType_CONNECTION_MADE, ConnStatusType_CONNECTION_NEEDED, ConnStatusType_CONNECTION_OK, ConnStatusType_CONNECTION_SETENV, ConnStatusType_CONNECTION_SSL_STARTUP, ConnStatusType_CONNECTION_STARTED, ExecStatusType, ExecStatusType_PGRES_BAD_RESPONSE, ExecStatusType_PGRES_COMMAND_OK, ExecStatusType_PGRES_COPY_BOTH, ExecStatusType_PGRES_COPY_IN, ExecStatusType_PGRES_COPY_OUT, ExecStatusType_PGRES_EMPTY_QUERY, ExecStatusType_PGRES_FATAL_ERROR, ExecStatusType_PGRES_NONFATAL_ERROR, ExecStatusType_PGRES_PIPELINE_ABORTED, ExecStatusType_PGRES_PIPELINE_SYNC, ExecStatusType_PGRES_SINGLE_TUPLE, ExecStatusType_PGRES_TUPLES_CHUNK, ExecStatusType_PGRES_TUPLES_OK, PGconn, PGresult, PQconnectdb, PQconsumeInput, PQexec, PQgetCopyData, PQgetResult, PQresultErrorMessage, PQresultStatus, PQstatus};
use std::ffi::{c_char, CStr, CString, c_void};
use std::slice;
use libc::{malloc, free, strlen};


pub mod libpq {
    include!(concat!(env!("OUT_DIR"), "/libpq_bindings.rs"));
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

pub unsafe fn start_replicating(conn: *mut PGconn) -> Result<i32, String> {
    let buf_siz = 1024;
    let mut buffer: *mut c_char = malloc(1024) as *mut c_char;
    println!("buffer@{}", buffer as isize);

    loop {
        if PQconsumeInput(conn) == 0 {
            println!("error!");
            return Err(String::from("failed to consume input"))
        }
        match PQgetCopyData(conn, &mut buffer, 0) {
            -1 => {
                println!("replication: done!");
                break;
            },
            -2 => println!("replication: copy failed!"),
            other => println!("copy data result: {}", other)
        }
        let length = buf_siz;

        // Convert the raw pointer to a slice of the calculated length
        let slice = slice::from_raw_parts(buffer as *const u8, length);

        // Print each byte in hexadecimal format
        for byte in slice {
            print!("{:02x} ", byte);
        }
        let cstr = CStr::from_ptr(buffer);
        println!("line: {}", cstr.to_str().expect("failed to parse string"));
    }
    free(buffer as *mut c_void);
    Ok(0)
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