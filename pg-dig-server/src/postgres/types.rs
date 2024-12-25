use std::ffi::{c_char, c_uint};
use std::slice;
use scroll::Pread;

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

/// XLogRecordHeader contains information about the record contained in the message.
#[repr(C)]
#[derive(Debug, Pread, PartialEq)]
pub struct XLogRecordHeader {
    pub xl_tot_len: u32,            /* total len of entire record */
    pub xl_xid: TransactionId,      /* xact id */
    pub xl_prev: u64,               /* ptr to previous record in log */
    pub xl_info: u8,                /* flag bits, see below */
    pub xl_rmid: RmgrId,            /* resource manager for this record */
    pub padding: [u8;2],            /* 2 bytes of padding here, initialize to zero */
    pub xl_crc: u32,                /* CRC for this record */
}

impl XLogRecordHeader {
    pub unsafe fn from_bytes(bytes: *const c_char) -> XLogRecordHeader {
        slice::from_raw_parts(bytes, size_of::<XLogRecordHeader>())
            .pread_with::<XLogRecordHeader>(0, scroll::LE)
            .expect("failed to read xlog record")
    }
}

/*
    typedef struct XLogRecordBlockHeader
    {
        uint8		id;				/* block reference ID */
        uint8		fork_flags;		/* fork within the relation, and flags */
        uint16		data_length;	/* number of payload bytes (not including page
                                     * image) */
    }
 */
#[repr(C)]
#[derive(Debug, Pread)]
pub struct XLogRecordBlockHeader {
    id: u8,
    fork_flags: u8,
    data_length: u16,
    image_header: Option<XLogRecordBlockImageHeader>,
    rel_file_locator: Option<RelFileLocator>,
    block_number: u32,
}

#[repr(C)]
#[derive(Debug, Pread)]
struct RelFileLocator {
    spc_oid: c_uint,        /* tablespace */
    db_oid: c_uint,         /* database */
    rel_number: c_uint,      /* relation */
}

#[repr(C)]
#[derive(Debug, Pread)]
struct XLogRecordBlockImageHeader {
    length: u16,
    hole_offset: u16,
    bimg_info: u8,
    hole_length: Option<u16>
}

impl XLogMessage {
    pub unsafe fn from_bytes(bytes: *const c_char) {
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
    pub unsafe fn from_bytes(buffer: *const c_char) -> XLogMessageHeader {
        slice::from_raw_parts(buffer as *const u8, size_of::<XLogMessageHeader>())
            .pread_with::<XLogMessageHeader>(0, scroll::LE)
            .expect("failed to read xlog record")
    }
}

/* Other Postgres Types */

#[repr(C)]
#[derive(Debug, Pread, PartialEq)]
pub struct TransactionId(pub u32);

#[repr(C)]
#[derive(Debug, Pread, PartialEq)]
pub struct RmgrId(pub u8);