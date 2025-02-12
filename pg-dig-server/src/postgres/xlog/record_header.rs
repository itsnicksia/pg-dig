use crate::postgres::common::transaction_id::TransactionId;
use bitflags::bitflags;
use scroll::Pread;
use std::fmt::Formatter;
use std::{fmt, slice};
use crate::postgres::common::rmgr::RmgrId;

/// XLogRecordHeader contains information about the record contained in the message.
#[repr(C)]
#[derive(Debug, Pread, PartialEq)]
pub struct XLogRecordHeader {
    pub xl_tot_len: u32,       /* total len of entire record */
    pub xl_xid: TransactionId, /* xact id */
    pub xl_prev: u64,          /* ptr to previous record in log */
    pub xl_info: u8,           /* flag bits, see below */
    pub xl_rmid: RmgrId,       /* resource manager for this record */
    pub xl_crc: u32,           /* CRC for this record */
}

impl XLogRecordHeader {
    pub unsafe fn read_flags(&self) -> XLogRecordHeaderFlags {
        XLogRecordHeaderFlags::from_bits_retain(self.xl_info)
    }

    pub fn read_rmgr_info_bytes(&self) -> u8 {
        (self.xl_info & 0xF0) >> 4
    }

    pub unsafe fn from_raw_ptr(bytes: *const u8) -> XLogRecordHeader {
        let record = slice::from_raw_parts(bytes, size_of::<XLogRecordHeader>())
            .pread_with::<XLogRecordHeader>(0, scroll::LE)
            .expect("failed to read xlog record");

        //println!("record: {:#?}", record);

        record
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct XLogRecordHeaderFlags: u8 {
        const XLR_SPECIAL_REL_UPDATE = 0b01;
        const XLR_CHECK_CONSISTENCY  = 0b10;
    }
}

impl fmt::Display for XLogRecordHeaderFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#" XLogRecordHeaderFlags(
    XLR_SPECIAL_REL_UPDATE: {}
    XLR_CHECK_CONSISTENCY: {}
)"#,
            self.contains(XLogRecordHeaderFlags::XLR_SPECIAL_REL_UPDATE),
            self.contains(XLogRecordHeaderFlags::XLR_CHECK_CONSISTENCY)
        )
    }
}
