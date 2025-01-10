use crate::postgres::platform::get_platform_endianness;
use crate::postgres::xlog::block_header::XLogRecordBlockHeader;
use crate::postgres::xlog::record_header::XLogRecordHeader;
use crate::postgres::xlog_parser::process_wal_record;
use scroll::Pread;
use std::slice;

/// XLogMessage contains the relevant parts of the replication message for monitoring.
///
/// We only read headers to avoid reading user data.
#[repr(C)]
#[derive(Debug)]
pub struct XLogMessage {
    message_header: XLogMessageHeader,
    wal_header: XLogRecordHeader,
    wal_block_headers: Vec<XLogRecordBlockHeader>
}

impl XLogMessage {
    #[allow(unused_variables)]
    pub unsafe fn from_ptr(bytes: *const u8) -> XLogMessage {
        let mut _offset = 1;

        let message_header = XLogMessageHeader::from_ptr(bytes.add(_offset));
        _offset += size_of::<XLogMessageHeader>();

        let wal_header = XLogRecordHeader::from_ptr(bytes.add(_offset));
        _offset += size_of::<XLogMessageHeader>();

        let wal_block_headers = process_wal_record(bytes.add(_offset));

        XLogMessage {
            message_header,
            wal_header,
            wal_block_headers
        }
    }
}

#[repr(C)]
#[derive(Debug, Pread)]
pub struct XLogMessageHeader {
    pub start_lsn: u64,
    pub end_lsn: u64,
    pub send_time: u64,
}

impl XLogMessageHeader {
    pub unsafe fn from_ptr(ptr: *const u8) -> XLogMessageHeader {
        slice::from_raw_parts(ptr, size_of::<XLogMessageHeader>())
            .pread_with::<XLogMessageHeader>(0, get_platform_endianness())
            .expect("failed to read xlog record")
    }
}
