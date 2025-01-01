use std::slice;
use scroll::Pread;
use crate::postgres::xlog::block_header::XLogRecordBlockHeader;
use crate::postgres::xlog::record_header::XLogRecordHeader;

/// XLogMessage contains the relevant parts of the replication message for monitoring.
///
/// We only read headers to avoid reading user data.
#[repr(C)]
#[derive(Debug, Pread)]
pub struct XLogMessage {
    header: XLogMessageHeader,
    record_header: XLogRecordHeader,
    //record_block_headers: Vec<XLogRecordBlockHeader>
}

impl XLogMessage {
    #[allow(unused_variables)]
    pub unsafe fn from_bytes(bytes: *const u8) {
        let message_header = XLogMessageHeader::from_bytes(bytes.add(1));
        let record_header = XLogRecordHeader::from_bytes(bytes.add(1 + size_of::<XLogMessageHeader>() ));
        let record_block_headers: Vec<XLogRecordBlockHeader> = vec![];
        loop {
            break;
        }

        todo!();
    }
}

#[repr(C)]
#[derive(Debug, Pread)]
pub struct XLogMessageHeader {
    pub data_start: u64,
    pub wal_end: u64,
    pub send_time: u64,
}

impl XLogMessageHeader {
    pub unsafe fn from_bytes(buffer: *const u8) -> XLogMessageHeader {
        slice::from_raw_parts(buffer as *const u8, size_of::<XLogMessageHeader>())
            .pread_with::<XLogMessageHeader>(0, scroll::LE)
            .expect("failed to read xlog record")
    }
}