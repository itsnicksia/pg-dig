use crate::postgres::common::transaction_id::{TransactionId, FIRST_NORMAL_TRANSACTION_ID, MAX_TRANSACTION_ID};
use crate::postgres::common::RelFileLocator;
use crate::postgres::xlog::block_header::XLogRecordBlockHeader;
use crate::postgres::xlog::constants::{XLR_BLOCK_ID_DATA_LONG, XLR_BLOCK_ID_DATA_SHORT, XLR_BLOCK_ID_ORIGIN, XLR_BLOCK_ID_TOPLEVEL_XID, XLR_MAX_BLOCK_ID};
use crate::postgres::xlog::record_header::XLogRecordHeader;

unsafe fn parse_normal_block_header(buffer: *const u8) -> (XLogRecordBlockHeader, usize) {
    let mut _offset = 0;

    let xlog_block_header = XLogRecordBlockHeader::from_bytes(buffer.add(_offset));

    _offset += match xlog_block_header.rel_file_locator {
        Some(_) => size_of::<XLogRecordBlockHeader>(),
        None => size_of::<XLogRecordBlockHeader>() - size_of::<RelFileLocator>()
    };

    (xlog_block_header, _offset)
}

pub unsafe fn process_wal_record(buffer: *const u8) -> Vec<XLogRecordBlockHeader> {
    let mut _offset = 0;
    let record = XLogRecordHeader::from_ptr(buffer);
    _offset += size_of::<XLogRecordHeader>();

    let TransactionId(xid) = record.xl_xid;

    match xid {
        0 .. FIRST_NORMAL_TRANSACTION_ID => {
            println!("skipping record with xid {}", xid);
            return Vec::new();
        },
        FIRST_NORMAL_TRANSACTION_ID ..= MAX_TRANSACTION_ID => { },
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

