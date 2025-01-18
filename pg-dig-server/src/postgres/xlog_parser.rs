use crate::postgres::common::transaction_id::{BOOTSTRAP_TRANSACTION_ID, INVALID_TRANSACTION_ID};
use crate::postgres::common::RelFileLocator;
use crate::postgres::xlog::block_header::XLogRecordBlockHeader;
use crate::postgres::xlog::constants::{
    XLR_BLOCK_ID_DATA_LONG, XLR_BLOCK_ID_DATA_SHORT, XLR_BLOCK_ID_ORIGIN,
    XLR_BLOCK_ID_TOPLEVEL_XID, XLR_MAX_BLOCK_ID,
};
use crate::postgres::xlog::record_header::XLogRecordHeader;
use crate::stop;
use log::info;

unsafe fn parse_normal_block_header(buffer: *const u8) -> (XLogRecordBlockHeader, usize) {
    let mut _offset = 0;

    let xlog_block_header = XLogRecordBlockHeader::from_raw_ptr(buffer.add(_offset));

    let header_size = match xlog_block_header.rel_file_locator {
        Some(_) => size_of::<XLogRecordBlockHeader>(),
        None => size_of::<XLogRecordBlockHeader>() - size_of::<RelFileLocator>(),
    };

    _offset += header_size;
    (xlog_block_header, _offset)
}

pub unsafe fn process_wal_record(buffer: *const u8) -> Vec<XLogRecordBlockHeader> {
    let mut _offset = 0;
    let record = XLogRecordHeader::from_raw_ptr(buffer);
    _offset += size_of::<XLogRecordHeader>();

    let txid = record.xl_xid;

    match txid {
        INVALID_TRANSACTION_ID => {
            info!("skipping record with invalid transaction id [0]");
            return Vec::new();
        }
        BOOTSTRAP_TRANSACTION_ID => {
            info!("skipping record with bootstrap transaction id [1]");
            return Vec::new();
        }
        _ => {}
    }

    let mut block_headers = Vec::new();
    /* peek at the block header id */
    loop {
        let block_id = *buffer.add(_offset);
        match block_id {
            0..XLR_MAX_BLOCK_ID => {
                let (block_header, block_header_size) = parse_normal_block_header(buffer);
                println!("{:#?}", block_header);
                block_headers.push(block_header);
                _offset += block_header_size;
            }
            XLR_BLOCK_ID_DATA_SHORT | XLR_BLOCK_ID_DATA_LONG => {
                println!("finished reading block headers");
                break;
            }
            XLR_BLOCK_ID_ORIGIN => {
                println!("skipping origin block header");
                break;
            }
            XLR_BLOCK_ID_TOPLEVEL_XID => println!("skipping toplevelxid block header"),
            _ => {
                println!(
                    "found invalid block id: {}. skipping rest of message.",
                    block_id
                );
                break;
            }
        }
    }

    block_headers
}
