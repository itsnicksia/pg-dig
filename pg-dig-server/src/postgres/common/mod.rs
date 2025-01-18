use scroll::Pread;
use std::ffi::c_uint;

pub mod lsn;
pub mod transaction_id;
pub mod rmgr;

#[repr(C)]
#[derive(Debug, Pread, PartialEq)]
pub struct RmgrId(pub u8);

#[repr(C)]
#[derive(Debug, Pread, PartialEq)]
pub struct RelFileLocator {
    pub spc_oid: c_uint,    /* tablespace */
    pub db_oid: c_uint,     /* database */
    pub rel_number: c_uint, /* relation */
}
