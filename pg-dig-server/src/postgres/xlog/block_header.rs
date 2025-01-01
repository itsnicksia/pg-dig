use std::{fmt, slice};
use std::fmt::Formatter;
use bitflags::bitflags;
use crate::postgres::common::RelFileLocator;
use crate::postgres::xlog::block_image_header::XLogRecordBlockImageHeader;

#[repr(C)]
#[derive(Debug)]
pub struct XLogRecordBlockHeader {
    id: u8,
    fork_flags: u8,
    data_length: u16,
    image_header: Option<XLogRecordBlockImageHeader>,
    rel_file_locator: Option<RelFileLocator>,
    block_number: u32,
}

impl XLogRecordBlockHeader {
    pub unsafe fn read_flags(&self) -> XLogRecordBlockHeaderFlags {
        XLogRecordBlockHeaderFlags::from_bits_retain(self.fork_flags)
    }

    pub unsafe fn from_bytes(bytes: *const u8) -> XLogRecordBlockHeader {
        let mut _offset = 0;
        /* block reference ID */
        let id = *bytes;
        _offset += size_of::<u8>();

        /* fork within the relation, and flags
         * The fork number fits in the lower 4 bits in the fork_flags field. The upper
         * bits are used for flags.
        */
        let fork_flags = *bytes.add(_offset);
        _offset += size_of::<u8>();

        /* number of payload bytes (not including page image) */
        let data_length = u16::from_le_bytes(
            slice::from_raw_parts(bytes.add(_offset), size_of::<u16>())
                .try_into()
                .expect("failed to parse data_length"));
        _offset += size_of::<u16>();

        let flags = XLogRecordBlockHeaderFlags::from_bits_retain(fork_flags);
        /* If BKPBLOCK_HAS_IMAGE, an XLogRecordBlockImageHeader struct follows */
        /* If BKPBLOCK_SAME_REL is not set, a RelFileLocator follows */

        let image_header = match flags.contains(XLogRecordBlockHeaderFlags::BKPBLOCK_HAS_IMAGE) {
            true =>  Some(XLogRecordBlockImageHeader::from_bytes(bytes.add(_offset))),
            false => None
        };

        XLogRecordBlockHeader {
            id,
            fork_flags,
            data_length,
            image_header,
            rel_file_locator: None,
            block_number: 0,
        }
    }
}

bitflags! {

    /*
        #define BKPBLOCK_FORK_MASK	0x0F
        #define BKPBLOCK_FLAG_MASK	0xF0
        #define BKPBLOCK_HAS_IMAGE	0x10	/* block data is an XLogRecordBlockImage */
        #define BKPBLOCK_HAS_DATA	0x20
        #define BKPBLOCK_WILL_INIT	0x40	/* redo will re-init the page */
        #define BKPBLOCK_SAME_REL	0x80	/* RelFileLocator omitted, same as previous */
     */
    #[derive(Debug)]
    pub struct XLogRecordBlockHeaderFlags: u8 {
        const BKPBLOCK_HAS_IMAGE    = 0x10;
        const BKPBLOCK_HAS_DATA     = 0x20;
        const BKPBLOCK_WILL_INIT    = 0x40;
        const BKPBLOCK_SAME_REL	    = 0x80;
    }
}

impl fmt::Display for XLogRecordBlockHeaderFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
               r#" XLogBlockHeaderFlags(
    raw: {:08b}
    BKPBLOCK_HAS_IMAGE: {}
    BKPBLOCK_HAS_DATA: {}
    BKPBLOCK_WILL_INIT: {}
    BKPBLOCK_SAME_REL: {}
)"#,
               self.bits(),
               self.contains(XLogRecordBlockHeaderFlags::BKPBLOCK_HAS_IMAGE),
               self.contains(XLogRecordBlockHeaderFlags::BKPBLOCK_HAS_DATA),
               self.contains(XLogRecordBlockHeaderFlags::BKPBLOCK_WILL_INIT),
               self.contains(XLogRecordBlockHeaderFlags::BKPBLOCK_SAME_REL)
        )
    }
}