use bitflags::bitflags;
use std::fmt::Formatter;
use std::{fmt, slice};

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
#[derive(Debug, PartialEq)]
pub struct XLogRecordBlockImageHeader {
    pub length: u16,
    pub hole_offset: u16,
    pub bimg_info: u8,
    pub padding: u8,
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
                .expect("failed to parse length"),
        );
        _offset += size_of::<u16>();

        let hole_offset = u16::from_le_bytes(
            slice::from_raw_parts(bytes.add(_offset), size_of::<u16>())
                .try_into()
                .expect("failed to parse hole_offset"),
        );
        _offset += size_of::<u16>();

        let bimg_info = *bytes.add(_offset);
        _offset += size_of::<u8>();

        XLogRecordBlockImageHeader {
            length,
            hole_offset,
            bimg_info,
            padding: 0,
        }
    }
}

bitflags! {
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

impl fmt::Display for XLogRecordBlockImageHeaderFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
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
