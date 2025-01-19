use crate::postgres::common::lsn::Lsn;
use crate::postgres::xlog::block_header::XLogRecordBlockHeader;
use crate::postgres::xlog_parser::process_wal_record;
use scroll::Pread;
use std::{fmt, slice};
use std::fmt::Formatter;
use crate::postgres::common::rmgr;
use crate::postgres::common::rmgr::get_simple_rmgr_info;
use crate::postgres::xlog::record_header::XLogRecordHeader;

/// XLogMessage contains the relevant parts of the replication message for monitoring.
///
/// We only read headers to avoid reading user data.
#[repr(C)]
pub struct XLogMessage {
    message_header: XLogMessageHeader,
    pub wal_header: XLogRecordHeader,
    pub wal_block_headers: Vec<XLogRecordBlockHeader>,
}

impl fmt::Display for XLogMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"
message:
    start_lsn: {}
    end_lsn: {}
    message_time: {}
wal_header:
    transaction id: {}
    resource manager: {} ({})
block_headers:
    {}
"#,
            Lsn::from_u64(self.message_header.start_lsn),
            Lsn::from_u64(self.message_header.end_lsn),
            "NYI",
            self.wal_header.xl_xid.0.to_string(),
            get_simple_rmgr_info(self.wal_header.xl_rmid.0, self.wal_header.read_rmgr_info_bytes()).rmgr_name,
            self.wal_header.xl_rmid.0,
            self.wal_block_headers.iter().map(|block_header| format!("{})", block_header)).collect::<Vec<_>>().join("\n    ")
        )
    }
}

impl XLogMessage {
    pub fn get_block_numbers(&self) -> Vec<u32> {
        self.wal_block_headers
            .iter()
            .map(|header| header.block_number)
            .collect()
    }

    pub unsafe fn from_ptr(ptr: *const u8) -> XLogMessage {
        let mut _offset = 1;

        let message_header = XLogMessageHeader::from_raw_ptr(ptr.add(_offset));
        _offset += size_of::<XLogMessageHeader>();

        let wal_header = XLogRecordHeader::from_raw_ptr(ptr.add(_offset));
        _offset += size_of::<XLogRecordHeader>();

        let wal_block_headers = process_wal_record(ptr.add(_offset));

        XLogMessage {
            message_header,
            wal_header,
            wal_block_headers,
        }
    }
}

#[repr(C)]
#[derive(Debug, Pread)]
pub struct XLogMessageHeader {
    // The first LSN of this message
    pub start_lsn: u64,

    // The most recent database LSN
    pub end_lsn: u64,

    // When it was sent
    pub send_time: u64,
}

impl XLogMessageHeader {
    pub unsafe fn from_raw_ptr(ptr: *const u8) -> XLogMessageHeader {
        slice::from_raw_parts(ptr, size_of::<XLogMessageHeader>())
            .pread_with::<XLogMessageHeader>(0, scroll::BE)
            .expect("failed to read xlog record")
    }
}
