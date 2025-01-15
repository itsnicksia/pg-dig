#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
use crate::postgres::bindings::{
    ConnStatusType, ConnStatusType_CONNECTION_ALLOCATED, ConnStatusType_CONNECTION_AUTH_OK,
    ConnStatusType_CONNECTION_AWAITING_RESPONSE, ConnStatusType_CONNECTION_BAD,
    ConnStatusType_CONNECTION_CHECK_STANDBY, ConnStatusType_CONNECTION_CHECK_TARGET,
    ConnStatusType_CONNECTION_CHECK_WRITABLE, ConnStatusType_CONNECTION_CONSUME,
    ConnStatusType_CONNECTION_GSS_STARTUP, ConnStatusType_CONNECTION_MADE,
    ConnStatusType_CONNECTION_NEEDED, ConnStatusType_CONNECTION_OK,
    ConnStatusType_CONNECTION_SETENV, ConnStatusType_CONNECTION_SSL_STARTUP,
    ConnStatusType_CONNECTION_STARTED, ExecStatusType, ExecStatusType_PGRES_BAD_RESPONSE,
    ExecStatusType_PGRES_COMMAND_OK, ExecStatusType_PGRES_COPY_BOTH, ExecStatusType_PGRES_COPY_IN,
    ExecStatusType_PGRES_COPY_OUT, ExecStatusType_PGRES_EMPTY_QUERY,
    ExecStatusType_PGRES_FATAL_ERROR, ExecStatusType_PGRES_NONFATAL_ERROR,
    ExecStatusType_PGRES_PIPELINE_ABORTED, ExecStatusType_PGRES_PIPELINE_SYNC,
    ExecStatusType_PGRES_SINGLE_TUPLE, ExecStatusType_PGRES_TUPLES_CHUNK,
    ExecStatusType_PGRES_TUPLES_OK, PGconn, PQgetResult, PQresultErrorMessage, PQresultStatus,
};
use std::ffi::CStr;

pub unsafe fn print_status(conn: *mut PGconn) {
    let result = PQgetResult(conn);
    let raw_error_message = CStr::from_ptr(PQresultErrorMessage(result))
        .to_str()
        .expect("failed to parse error message");

    let result_status = PQresultStatus(result);

    print!("result: {}", friendly_exec_status(result_status));

    if !raw_error_message.is_empty() {
        println!(", error: {}", raw_error_message)
    }

    println!();
    println!();
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
