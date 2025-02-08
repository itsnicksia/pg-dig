#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::postgres::bindings::*;
use crate::postgres::pg_conn::friendly_exec_status;
use crate::postgres::query::exec;
use crate::postgres::xlog_message::XLogMessage;
use std::ffi::{c_char, CStr};

const replication_slot_name: &str = "physical";
const start_lsn: &str = "0/1000000";

pub unsafe fn start_replication(conn: *mut PGconn) -> Result<(), String> {
    let stmt = format!("START_REPLICATION SLOT {} PHYSICAL {}", replication_slot_name, start_lsn);

    let result = exec(conn, stmt.as_str());

    match PQresultStatus(result) {
        ExecStatusType_PGRES_COPY_BOTH => Ok(()),
        other => Err(friendly_exec_status(other)),
    }
}

pub unsafe fn read_message(conn: *mut PGconn) -> Result<XLogMessage, String> {
    let mut buffer= vec![0; 1024];
    let mut buffer_ptr: *mut c_char = buffer.as_mut_ptr();

    loop {
        if PQconsumeInput(conn) == 0 {
            return Err(String::from("failed to consume input"))
        }

        // Handle errors
        match PQgetCopyData(conn, &mut buffer_ptr, 0) {
            length if length > 0 => {},
            -1 => return Err(String::from("end of stream")),
            -2 => {
                let error = PQerrorMessage(conn);
                let error_message = format!("replication failed!. Reason: {}", CStr::from_ptr(error).to_str().expect("failed to parse error message"));
                return Err(String::from(error_message));
            },
            unknown_code => panic!("unknown code from PQgetCopyData: {}", unknown_code)
        };

        // Handle message
        match *buffer_ptr as u8 as char {
            'w' => match XLogMessage::from_ptr(buffer_ptr.add(1) as *const u8) {
                Ok(result) => {
                    return Ok(result)
                },
                Err(msg)=> println!("{}", msg)
            } ,
            'k' => println!("*keep-alive*"),
            record_code => return Err(String::from(format!("unexpected record type: {}", record_code)))
        };
    }
}


