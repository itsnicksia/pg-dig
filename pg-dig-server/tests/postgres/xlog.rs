use crate::postgres::test_data::TEST_BUFFER;
use pg_dig_server::postgres::common::transaction_id::TransactionId;
use pg_dig_server::postgres::common::{RelFileLocator, RmgrId};
use pg_dig_server::postgres::xlog::block_header::XLogRecordBlockHeader;
use pg_dig_server::postgres::xlog::block_image_header::XLogRecordBlockImageHeader;
use pg_dig_server::postgres::xlog::record_header::XLogRecordHeader;
use pg_dig_server::postgres::xlog_message::XLogMessageHeader;

#[test]
fn xlog_header_from_buffer() {
    unsafe {
        let offset = 1;
        let record = XLogMessageHeader::from_raw_ptr(TEST_BUFFER.as_ptr().add(offset));

        assert_eq!(
            record.start_lsn, 9235850398613372928,
            "data_start should be 9235850398613372928"
        );
    }
}

#[test]
fn xlog_record_from_buffer() {
    unsafe {
        let offset = size_of::<XLogMessageHeader>() + 1;
        let record = XLogRecordHeader::from_raw_ptr(TEST_BUFFER.as_ptr().add(offset));

        assert_eq!(record.xl_tot_len, 42, "total length should be 42");
        assert_eq!(
            record.xl_xid,
            TransactionId(746),
            "transaction id should be TransactionId(746)"
        );
        assert_eq!(
            record.xl_prev, 22_359_112,
            "previous xlog ptr should be 22359112"
        );
        assert_eq!(record.xl_rmid, RmgrId(8), "rmid should be RmgrId(8)");
        assert_eq!(record.xl_crc, 101_056_512, "crc should be 101056512");
    }
}

// "block_ref": "blkref #0: rel 1663/1/1249 fork fsm blk 2 (FPW); hole: offset: 0, length: 0"
#[test]
pub fn xlog_block_header_from_buffer() {
    let buffer = [
        0x0, 0x11, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x2, 0x7F, 0x6, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0,
        0xDF, 0x4, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0,
    ];
    let expected = XLogRecordBlockHeader {
        id: 0,
        fork_flags: 17, // Eh?
        data_length: 0,
        image_header: Some(XLogRecordBlockImageHeader {
            length: 8192,
            hole_offset: 0,
            bimg_info: 2,
            padding: 0,
            hole_length: None,
        }),
        rel_file_locator: Some(RelFileLocator {
            spc_oid: 1663,
            db_oid: 1,
            rel_number: 1247,
        }),
        block_number: 2,
    };

    unsafe {
        let record = XLogRecordBlockHeader::from_raw_ptr(buffer.as_ptr());
        assert_eq!(record, expected);
    }
}
