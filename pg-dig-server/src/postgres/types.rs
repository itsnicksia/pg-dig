use std::ffi::{c_uint};
use std::{fmt, slice};
use std::fmt::Formatter;
use bitflags::bitflags;
use scroll::Pread;

bitflags! {
    #[derive(Debug)]
    pub struct XLogRecordFlags: u8 {
        const XLR_SPECIAL_REL_UPDATE = 0b01;
        const XLR_CHECK_CONSISTENCY  = 0b10;
    }

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

    /*
        /* Information stored in bimg_info */
        #define BKPIMAGE_HAS_HOLE		0x01	/* page image has "hole" */
        #define BKPIMAGE_APPLY			0x02	/* page image should be restored
                                                 * during replay */
        /* compression methods supported */
        #define BKPIMAGE_COMPRESS_PGLZ	0x04
        #define BKPIMAGE_COMPRESS_LZ4	0x08
        #define BKPIMAGE_COMPRESS_ZSTD	0x10
     */
    #[derive(Debug)]
    pub struct XLogRecordBlockImageHeaderFlags: u8 {
        const BKPIMAGE_HAS_HOLE         = 0x01;
        const BKPIMAGE_APPLY            = 0x02;
        const BKPIMAGE_COMPRESS_PGLZ    = 0x04;
        const BKPIMAGE_COMPRESS_LZ4	    = 0x08;
        const BKPIMAGE_COMPRESS_ZSTD	= 0x10;
    }
}

impl fmt::Display for XLogRecordFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
               r#" XLogBlockHeaderFlags(
    raw: {:08b}
    XLR_SPECIAL_REL_UPDATE: {}
    XLR_CHECK_CONSISTENCY: {}
)"#,
               self.bits(),
               self.contains(XLogRecordFlags::XLR_SPECIAL_REL_UPDATE),
               self.contains(XLogRecordFlags::XLR_CHECK_CONSISTENCY)
        )
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

impl fmt::Display for XLogRecordBlockImageHeaderFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
               r#" XLogRecordBlockImageHeaderFlags(
    raw: {:08b}
    BKPIMAGE_HAS_HOLE: {}
    BKPIMAGE_APPLY: {}
    BKPIMAGE_COMPRESS_PGLZ: {}
    BKPIMAGE_COMPRESS_LZ4: {}
    BKPIMAGE_COMPRESS_ZSTD: {}
)"#,
               self.bits(),
               self.contains(XLogRecordBlockImageHeaderFlags::BKPIMAGE_HAS_HOLE),
               self.contains(XLogRecordBlockImageHeaderFlags::BKPIMAGE_APPLY),
               self.contains(XLogRecordBlockImageHeaderFlags::BKPIMAGE_COMPRESS_PGLZ),
               self.contains(XLogRecordBlockImageHeaderFlags::BKPIMAGE_COMPRESS_LZ4),
               self.contains(XLogRecordBlockImageHeaderFlags::BKPIMAGE_COMPRESS_ZSTD)
        )
    }
}

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
    pub unsafe fn read_flags(self: XLogRecordHeader) -> XLogRecordFlags {
         XLogRecordFlags::from_bits_retain(self.xl_info)
    }

    pub unsafe fn from_bytes(bytes: *const u8) -> XLogRecordHeader {
        slice::from_raw_parts(bytes, size_of::<XLogRecordHeader>())
            .pread_with::<XLogRecordHeader>(0, scroll::LE)
            .expect("failed to read xlog record")
    }
}

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

/*
typedef struct XLogRecordBlockImageHeader
{
	uint16		length;			/* number of page image bytes */
	uint16		hole_offset;	/* number of bytes before "hole" */
	uint8		bimg_info;		/* flag bits, see below */

	/*
	 * If BKPIMAGE_HAS_HOLE and BKPIMAGE_COMPRESSED(), an
	 * XLogRecordBlockCompressHeader struct follows.
	 */
} XLogRecordBlockImageHeader;
 */
#[repr(C)]
#[derive(Debug)]
pub struct XLogRecordBlockImageHeader {
    length: u16,
    hole_offset: u16,
    bimg_info: u8,
    // FIXME: Breaks pread
    //hole_length: Option<u16>
}

impl XLogRecordBlockImageHeader {
    pub unsafe fn read_flags(&self) -> XLogRecordBlockImageHeaderFlags {
        XLogRecordBlockImageHeaderFlags::from_bits_retain(self.bimg_info)
    }

    pub unsafe fn from_bytes(bytes: *const u8) -> XLogRecordBlockImageHeader {
        let mut _offset = 0;

        let length = u16::from_le_bytes(
            slice::from_raw_parts(bytes, size_of::<u16>())
                .try_into()
                .expect("failed to parse length"));
        _offset += size_of::<u16>();

        let hole_offset = u16::from_le_bytes(
            slice::from_raw_parts(bytes.add(_offset), size_of::<u16>())
                .try_into()
                .expect("failed to parse hole_offset"));
        _offset += size_of::<u16>();

        let bimg_info = *bytes.add(_offset);

        XLogRecordBlockImageHeader {
            length,
            hole_offset,
            bimg_info,
        }
    }
}

#[repr(C)]
#[derive(Debug, Pread)]
struct RelFileLocator {
    spc_oid: c_uint,        /* tablespace */
    db_oid: c_uint,         /* database */
    rel_number: c_uint,      /* relation */
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

/* Other Postgres Types */

#[repr(C)]
#[derive(Debug, Pread, PartialEq)]
pub struct TransactionId(pub u32);

#[repr(C)]
#[derive(Debug, Pread, PartialEq)]
pub struct RmgrId(pub u8);