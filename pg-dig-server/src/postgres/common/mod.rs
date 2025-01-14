use std::ffi::c_uint;
use scroll::Pread;

pub mod lsn;
pub mod transaction_id;

#[repr(C)]
#[derive(Debug, Pread, PartialEq)]
pub struct RmgrId(pub u8);

#[repr(C)]
#[derive(Debug, Pread)]
pub struct RelFileLocator {
    spc_oid: c_uint,        /* tablespace */
    db_oid: c_uint,         /* database */
    pub rel_number: c_uint,      /* relation */
}

pub fn print_hex_bytes(ptr: *const u8, num_bytes: usize) {
    unsafe {
        for i in 0..num_bytes {
            let byte = ptr.add(i).read(); // Read the byte at the offset
            print!("{:02X} ", byte);     // Print the byte in hexadecimal format
        }
    }
    println!(); // Newline after printing all bytes
}