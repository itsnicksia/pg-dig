use std::ffi::c_char;
use std::slice;
use scroll::Pread;

#[repr(C)]
#[derive(Debug, Pread, PartialEq)]
pub struct TransactionId(pub u32);

#[repr(C)]
#[derive(Debug, Pread, PartialEq)]
pub struct RmgrId(pub u8);

#[repr(C)]
#[derive(Debug, Pread, PartialEq)]
pub struct XLogRecord {
    pub xl_tot_len: u32,            /* total len of entire record */
    pub xl_xid: TransactionId,      /* xact id */
    pub xl_prev: u64,               /* ptr to previous record in log */
    pub xl_info: u8,                /* flag bits, see below */
    pub xl_rmid: RmgrId,            /* resource manager for this record */
    pub padding: [u8;2],            /* 2 bytes of padding here, initialize to zero */
    pub xl_crc: u32,                /* CRC for this record */
    /* XLogRecordBlockHeaders and XLogRecordDataHeader follow, no padding */
}

impl XLogRecord {
    pub unsafe fn from_buffer(buffer: *mut c_char) -> XLogRecord {
        slice::from_raw_parts(buffer as *const u8, size_of::<XLogHeader>())
            .pread_with::<XLogRecord>(0, scroll::LE)
            .expect("failed to read xlog record")
    }
}

/*
    dataStart = pq_getmsgint64(&incoming_message);
    walEnd = pq_getmsgint64(&incoming_message);
    sendTime = pq_getmsgint64(&incoming_message);
    ProcessWalSndrMessage(walEnd, sendTime);
 */
#[repr(C)]
#[derive(Debug, Pread)]
pub struct XLogHeader {
    pub data_start: u64,
    pub wal_end: u64,
    pub send_time: u64,
}

/*
        typedef struct XLogRecordBlockHeader
    {
        uint8		id;				/* block reference ID */
        uint8		fork_flags;		/* fork within the relation, and flags */
        uint16		data_length;	/* number of payload bytes (not including page
                                     * image) */

        /* If BKPBLOCK_HAS_IMAGE, an XLogRecordBlockImageHeader struct follows */
        /* If BKPBLOCK_SAME_REL is not set, a RelFileNode follows */
        /* BlockNumber follows */
    } XLogRecordBlockHeader;
 */
#[repr(C)]
#[derive(Debug, Pread)]
pub struct XLogRecordBlockHeader {
    id: u8,
    fork_flags: u8,
    data_length: u16,
}