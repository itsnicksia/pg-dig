#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::libpq::{ConnStatusType, ConnStatusType_CONNECTION_ALLOCATED, ConnStatusType_CONNECTION_AUTH_OK, ConnStatusType_CONNECTION_AWAITING_RESPONSE, ConnStatusType_CONNECTION_BAD, ConnStatusType_CONNECTION_CHECK_STANDBY, ConnStatusType_CONNECTION_CHECK_TARGET, ConnStatusType_CONNECTION_CHECK_WRITABLE, ConnStatusType_CONNECTION_CONSUME, ConnStatusType_CONNECTION_GSS_STARTUP, ConnStatusType_CONNECTION_MADE, ConnStatusType_CONNECTION_NEEDED, ConnStatusType_CONNECTION_OK, ConnStatusType_CONNECTION_SETENV, ConnStatusType_CONNECTION_SSL_STARTUP, ConnStatusType_CONNECTION_STARTED, PGconn, PQgetResult, PQresultErrorMessage, PQstatus};
use std::ffi::CStr;
pub mod libpq {
    include!(concat!(env!("OUT_DIR"), "/libpq_bindings.rs"));
}

pub unsafe fn print_status(conn: *mut PGconn) {
    let conn_status = friendly_conn_status(PQstatus(conn));
    let result = PQgetResult(conn);
    let raw_error_message = CStr::from_ptr(PQresultErrorMessage(result)).to_str().unwrap();
    let error_message = if raw_error_message.is_empty() { "none" } else { raw_error_message };
    println!("{} | error: {}", conn_status, error_message);
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