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
