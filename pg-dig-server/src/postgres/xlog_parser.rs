use crate::postgres::common::info::Info;
use crate::postgres::common::{RelFileLocator, TransactionId};
use crate::postgres::xlog::block_header::XLogRecordBlockHeader;
use crate::postgres::xlog::constants::{XLR_BLOCK_ID_DATA_LONG, XLR_BLOCK_ID_DATA_SHORT, XLR_BLOCK_ID_ORIGIN, XLR_BLOCK_ID_TOPLEVEL_XID, XLR_MAX_BLOCK_ID};
use crate::postgres::xlog::record_header::XLogRecordHeader;
use crate::postgres::xlog_message::XLogMessageHeader;
use scroll::Pread;
use std::ffi::c_char;
use std::slice;
use crate::postgres::common::transaction_id::{FIRST_NORMAL_TRANSACTION_ID, MAX_TRANSACTION_ID};

pub unsafe fn parse_message<C>(c_buffer: *const c_char) {
    let buffer: *const u8 = c_buffer as *const u8;

    // The first byte of the message is the message-type, i.e. "w" for wal, "k" for keep-alive.
    let mut offset = 1;

    process_wal_record_header(buffer.add(offset));
    offset += size_of::<XLogMessageHeader>();

    process_wal_record(buffer.add(offset));
}

unsafe fn process_wal_record_header(buffer: *const u8) {
    let header_slice = slice::from_raw_parts(buffer, size_of::<XLogMessageHeader>());

    let xlog_header = header_slice
        .pread_with::<XLogMessageHeader>(0, scroll::BE)
        .expect("failed to read xlog record header");
}

unsafe fn parse_normal_block_header(buffer: *const u8) -> (XLogRecordBlockHeader, usize) {
    let mut _offset = 0;

    let xlog_block_header = XLogRecordBlockHeader::from_bytes(buffer.add(_offset));

    _offset += match xlog_block_header.rel_file_locator {
        Some(_) => size_of::<XLogRecordBlockHeader>(),
        None => size_of::<XLogRecordBlockHeader>() - size_of::<RelFileLocator>()
    };

    (xlog_block_header, _offset)
}

pub unsafe fn process_wal_record<C>(buffer: *const u8) -> Vec<XLogRecordBlockHeader> {
    let mut _offset = 0;
    let record = XLogRecordHeader::from_bytes(buffer);
    _offset += size_of::<XLogRecordHeader>();

    match record.xl_xid {
        0 .. FIRST_NORMAL_TRANSACTION_ID => {
            println!("skipping record with xid {}", record.xl_xid);
            return Vec::new();
        },
        FIRST_NORMAL_TRANSACTION_ID .. MAX_TRANSACTION_ID => { },
        _ => panic!("xid out of range")
    }

    let mut block_headers = Vec::new();
    /* peek at the block header id */
    loop {
        let block_id = *buffer.add(_offset);
        match block_id {
            0..XLR_MAX_BLOCK_ID => {
                let (block_header, block_header_size) = parse_normal_block_header(buffer);
                block_headers.push(block_header);
                _offset += block_header_size;
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

    block_headers
}

