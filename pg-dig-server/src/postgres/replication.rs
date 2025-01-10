#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::postgres::bindings::*;
use crate::postgres::common::info::Info;
use crate::postgres::common::{RelFileLocator, TransactionId};
use crate::postgres::query::exec;
use crate::postgres::xlog_message::XLogMessageHeader;
use crate::postgres::xlog::block_header::XLogRecordBlockHeader;
use crate::postgres::xlog::constants::{XLR_BLOCK_ID_DATA_LONG, XLR_BLOCK_ID_DATA_SHORT, XLR_BLOCK_ID_ORIGIN, XLR_BLOCK_ID_TOPLEVEL_XID, XLR_MAX_BLOCK_ID};
use crate::postgres::xlog::record_header::XLogRecordHeader;
use scroll::Pread;
use std::ffi::{c_char, CStr};
use std::slice;
use crate::postgres::pg_conn::friendly_exec_status;
use crate::postgres::xlog_parser::parse_message;

const replication_slot_name: &str = "physical";
const start_lsn: &str = "0/4377DED8";

pub unsafe fn start_replication(conn: *mut PGconn) -> Result<(), String> {
    let stmt = format!("START_REPLICATION SLOT {} PHYSICAL {}", replication_slot_name, start_lsn);

    let result = exec(conn, stmt.as_str());

    match PQresultStatus(result) {
        ExecStatusType_PGRES_COPY_BOTH => Ok(()),
        other => Err(friendly_exec_status(other)),
    }
}

pub unsafe fn read_message<C>(conn: *mut PGconn) -> Result<Info, String> {
    let mut buffer: *mut c_char = vec![0; 1024].as_mut_ptr();

    loop {
        if PQconsumeInput(conn) == 0 {
            return Err(String::from("failed to consume input"))
        }

        match PQgetCopyData(conn, &mut buffer, 0) {
            length if length > 0 => { },
            -1 => return Err(String::from("end of stream")),
            -2 => {
                let error = PQerrorMessage(conn);
                let error_message = format!("replication failed!. Reason: {}", CStr::from_ptr(error).to_str().expect("failed to parse error message"));
                return Err(String::from(error_message));
            },
            unknown_code => panic!("unknown code from PQgetCopyData: {}", unknown_code)
        };

        match *buffer as u8 as char {
            'w' => parse_message(&mut *buffer),
            'k' => println!("*keep-alive*"),
            record_code => panic!("unexpected record type: {}", record_code)
        }
    }
}


