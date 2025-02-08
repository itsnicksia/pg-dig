#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt;
use scroll::Pread;

#[repr(C)]
#[derive(Clone, Debug, Pread, PartialEq)]
pub struct RmgrId(pub u8);

#[repr(u8)]
pub enum ResourceManager {
    XLOG = 0,
    Transaction = 1,
    Storage = 2,
    CLOG = 3,
    Database = 4,
    Tablespace = 5,
    MultiXact = 6,
    RelMap = 7,
    Standby = 8,
    Heap2 = 9,
    Heap = 10,
    Btree = 11,
    Hash = 12,
    Gin = 13,
    Gist = 14,
    Sequence = 15,
    SPGist = 16,
    BRIN = 17,
    CommitTs = 18,
    ReplicationOrigin = 19,
    Generic = 20,
    LogicalMessage = 21,
}

impl ResourceManager {
    pub(crate) fn get_record_type(&self, rmgr_info: u8) -> String {
        "NYI".to_string()
    }
}

pub struct SimpleRmgrInfo {
    pub rmgr_name: String,
    pub record_type: String
}

pub fn get_simple_rmgr_info(rmgr_id: RmgrId, rmgr_info: u8) -> SimpleRmgrInfo {
    let resource_manager = ResourceManager::try_from(rmgr_id).expect("invalid rmgr_id");
    let record_type = resource_manager.get_record_type(rmgr_info);
    SimpleRmgrInfo {
        rmgr_name: resource_manager.to_string(),
        record_type: "NYI".to_string()
    }
}

impl TryFrom<RmgrId> for ResourceManager {
    type Error = &'static str;

    fn try_from(value: RmgrId) -> Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(ResourceManager::XLOG),
            1 => Ok(ResourceManager::Transaction),
            2 => Ok(ResourceManager::Storage),
            3 => Ok(ResourceManager::CLOG),
            4 => Ok(ResourceManager::Database),
            5 => Ok(ResourceManager::Tablespace),
            6 => Ok(ResourceManager::MultiXact),
            7 => Ok(ResourceManager::RelMap),
            8 => Ok(ResourceManager::Standby),
            9 => Ok(ResourceManager::Heap2),
            10 => Ok(ResourceManager::Heap),
            11 => Ok(ResourceManager::Btree),
            12 => Ok(ResourceManager::Hash),
            13 => Ok(ResourceManager::Gin),
            14 => Ok(ResourceManager::Gist),
            15 => Ok(ResourceManager::Sequence),
            16 => Ok(ResourceManager::SPGist),
            17 => Ok(ResourceManager::BRIN),
            18 => Ok(ResourceManager::CommitTs),
            19 => Ok(ResourceManager::ReplicationOrigin),
            20 => Ok(ResourceManager::Generic),
            21 => Ok(ResourceManager::LogicalMessage),
            id => Err("Invalid value for ResourceManager"),
        }
    }
}

impl fmt::Display for ResourceManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ResourceManager::XLOG => "XLOG",
            ResourceManager::Transaction => "Transaction",
            ResourceManager::Storage => "Storage",
            ResourceManager::CLOG => "CLOG",
            ResourceManager::Database => "Database",
            ResourceManager::Tablespace => "Tablespace",
            ResourceManager::MultiXact => "MultiXact",
            ResourceManager::RelMap => "RelMap",
            ResourceManager::Standby => "Standby",
            ResourceManager::Heap2 => "Heap2",
            ResourceManager::Heap => "Heap",
            ResourceManager::Btree => "Btree",
            ResourceManager::Hash => "Hash",
            ResourceManager::Gin => "Gin",
            ResourceManager::Gist => "Gist",
            ResourceManager::Sequence => "Sequence",
            ResourceManager::SPGist => "SPGist",
            ResourceManager::BRIN => "BRIN",
            ResourceManager::CommitTs => "CommitTs",
            ResourceManager::ReplicationOrigin => "ReplicationOrigin",
            ResourceManager::Generic => "Generic",
            ResourceManager::LogicalMessage => "LogicalMessage",
        };
        write!(f, "{}", name)
    }
}


/* XLOG info values for XLOG rmgr */
pub const XLOG_CHECKPOINT_SHUTDOWN: u32 = 0x00;
pub const XLOG_CHECKPOINT_ONLINE: u32 = 0x10;
pub const XLOG_NOOP: u32 = 0x20;
pub const XLOG_NEXTOID: u32 = 0x30;
pub const XLOG_SWITCH: u32 = 0x40;
pub const XLOG_BACKUP_END: u32 = 0x50;
pub const XLOG_PARAMETER_CHANGE: u32 = 0x60;
pub const XLOG_RESTORE_POINT: u32 = 0x70;
pub const XLOG_FPW_CHANGE: u32 = 0x80;
pub const XLOG_END_OF_RECOVERY: u32 = 0x90;
pub const XLOG_FPI_FOR_HINT: u32 = 0xA0;
pub const XLOG_FPI: u32 = 0xB0;
