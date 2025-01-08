#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::postgres::bindings::*;
use crate::postgres::common::{RelFileLocator, TransactionId};
use crate::postgres::replication_message::XLogMessageHeader;
use crate::postgres::xlog::block_header::XLogRecordBlockHeader;
use crate::postgres::xlog::constants::{XLR_BLOCK_ID_DATA_LONG, XLR_BLOCK_ID_DATA_SHORT, XLR_BLOCK_ID_ORIGIN, XLR_BLOCK_ID_TOPLEVEL_XID, XLR_MAX_BLOCK_ID};
use crate::postgres::xlog::record_header::XLogRecordHeader;
use scroll::Pread;
use std::ffi::{c_char, CStr, CString};
use std::slice;
use crate::postgres::common::info::Info;

const replication_slot_name: &str = "physical";
const start_lsn: &str = "0/4377DED8";

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
    let stmt = format!("START_REPLICATION SLOT {} PHYSICAL {}", replication_slot_name, start_lsn);
    let result = exec(conn, stmt.as_str());
    match PQresultStatus(result) {
        ExecStatusType_PGRES_COPY_BOTH => Ok(()),
        other => Err(friendly_exec_status(other)),
    }
}

pub unsafe fn start_replicating<C>(conn: *mut PGconn, consumer: C) -> Result<i32, String>
    where C: Fn(Info) {
    let mut buffer: *mut c_char = vec![0; 1024].as_mut_ptr();
    loop {
        if PQconsumeInput(conn) == 0 {
            println!("error!");
            return Err(String::from("failed to consume input"))
        }
        match PQgetCopyData(conn, &mut buffer, 0) {
            length if length > 0 => { },
            -1 => panic!("replication: done!"),
            -2 => {
                let error = PQerrorMessage(conn);
                panic!("replication failed!. Reason: {}", CStr::from_ptr(error).to_str().expect("failed to parse error message"));
            },
            other => panic!("not yet implemented: {}", other)
        };

        match *buffer as u8 as char {
            'w' => process_wal_message(&mut *buffer, &consumer),
            'k' => println!("*keep-alive*"),
            other => panic!("unexpected record type: {}", other)
        }
    }
}

unsafe fn process_wal_message<C>(buffer: *const c_char, consumer: C) where C: Fn(Info) {
    let u8_buffer: *const u8 = buffer as *const u8;
    let mut offset = 1;
    process_wal_record_header(u8_buffer.add(offset));
    offset += size_of::<XLogMessageHeader>();

    process_wal_record(u8_buffer.add(offset), consumer);
}

#[allow(unused_variables)]
unsafe fn process_wal_record_header(buffer: *const u8) {
    let header_slice = slice::from_raw_parts(buffer, size_of::<XLogMessageHeader>());

    let xlog_header = header_slice
        .pread_with::<XLogMessageHeader>(0, scroll::BE)
        .expect("failed to read xlog record header");

    // println!("dataStart: {}", Lsn::from_u64(xlog_header.data_start));
    // println!("walEnd: {}", Lsn::from_u64(xlog_header.wal_end));
    // println!("sendTime: {}",
    //          DateTime::from_timestamp_micros(xlog_header.send_time.try_into().expect("sendTime too large"))
    //              .expect("error")
    //              /* FIXME: Not sure why it's off by exactly 30 years...? */
    //              .checked_add_months(Months::new(360)).unwrap()
    //              .checked_sub_days(Days::new(1)).unwrap()
    // );
}

#[allow(unused_variables)]
unsafe fn process_wal_record<C>(buffer: *const u8, consumer:C) where C: Fn(Info) {
    let mut _offset: usize = 0;
    let xlog_record = XLogRecordHeader::from_bytes(buffer);
    _offset += size_of::<XLogRecordHeader>();

    let xid = xlog_record.xl_xid;
    //println!("xid: {:?}", xid);

    match xid {
        TransactionId(0) => return,
        TransactionId(1) => return,
        _ => {}
    }

    /* peek at the block header id */
    loop {
        let block_id = *buffer.add(_offset);
        match block_id {
            0..XLR_MAX_BLOCK_ID => {
                let xlog_block_header = XLogRecordBlockHeader::from_bytes(buffer.add(_offset));
                let xlog_block_header_flags = xlog_block_header.read_flags();
                _offset += size_of::<u8>() + size_of::<u8>() + size_of::<u16>() + size_of::<u32>();
                if xlog_block_header.rel_file_locator.is_some() {
                    _offset += size_of::<RelFileLocator>();
                }
                println!("block written: {}", xlog_block_header.block_number);
                let fork_number = xlog_block_header.fork_flags >> 4;
                consumer(Info {
                    block_number: xlog_block_header.block_number,
                    table_name: match xlog_block_header.rel_file_locator {
                        Some(loc) => loc.rel_number.to_string(),
                        None => "unknown".to_string(),
                    },
                    fork_name: match fork_number {
                        0 => "main",
                        1 => "fsm",
                        2 => "vis",
                        3 => "init",
                        _ => panic!("invalid fork number: {}", fork_number),
                    }.to_string()
                });
            },
            XLR_BLOCK_ID_DATA_SHORT | XLR_BLOCK_ID_DATA_LONG => {
                println!("finished block headers: {}", block_id);
                break;
            },
            XLR_BLOCK_ID_ORIGIN => {
                println!("ignoring unwanted origin header ()");
                break;
            },
            XLR_BLOCK_ID_TOPLEVEL_XID => println!("ignoring unwanted toplevelxid header ()"),
            _ => {
                println!("found invalid block id: {}. skipping rest of message.", block_id);
                break;
            }
        }
    }


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
