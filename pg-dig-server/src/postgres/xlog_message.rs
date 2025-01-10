use std::ops::AddAssign;
use std::slice;
use scroll::Pread;
use crate::postgres::common::info::Info;
use crate::postgres::platform::get_platform_endianness;
use crate::postgres::xlog::block_header::XLogRecordBlockHeader;
use crate::postgres::xlog::record_header::XLogRecordHeader;
use crate::postgres::xlog_parser::process_wal_record;

/// XLogMessage contains the relevant parts of the replication message for monitoring.
///
/// We only read headers to avoid reading user data.
#[repr(C)]
#[derive(Debug)]
pub struct XLogMessage {
    header: XLogMessageHeader,
    record_header: XLogRecordHeader,
    record_block_headers: Vec<XLogRecordBlockHeader>
}

impl XLogMessage {
    #[allow(unused_variables)]
    pub unsafe fn from_bytes(bytes: *const u8) {
        let mut _offset = 1;

        let message_header = XLogMessageHeader::from_ptr(bytes.add(_offset));
        _offset += size_of::<XLogMessageHeader>();

        let record_header = XLogRecordHeader::from_bytes(bytes.add(_offset));
        _offset += size_of::<XLogMessageHeader>();

        let record_block_headers: Vec<XLogRecordBlockHeader> = process_wal_record(bytes.add(_offset));

        record_block_headers.iter().map(|block_header| { let fork_number = xlog_block_header.fork_flags >> 4;
            Info {
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
            }; })


        todo!();
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