#![allow(unused_variables)]
#![allow(dead_code)]
use phf::phf_map;

pub const RMGR_MAP: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "XLOG",
    1u8 => "Transaction",
    2u8 => "Storage",
    3u8 => "CLOG",
    4u8 => "Database",
    5u8 => "Tablespace",
    6u8 => "MultiXact",
    7u8 => "RelMap",
    8u8 => "Standby",
    9u8 => "Heap2",
    10u8 => "Heap",
    11u8 => "Btree",
    12u8 => "Hash",
    13u8 => "Gin",
    14u8 => "Gist",
    15u8 => "Sequence",
    16u8 => "SPGist",
    17u8 => "BRIN",
    18u8 => "CommitTs",
    19u8 => "ReplicationOrigin",
    20u8 => "Generic",
    21u8 => "LogicalMessage",
};

// PG_RMGR(RM_XLOG_ID, "XLOG", xlog_redo, xlog_desc, xlog_identify, NULL, NULL, NULL, xlog_decode)
// PG_RMGR(RM_XACT_ID, "Transaction", xact_redo, xact_desc, xact_identify, NULL, NULL, NULL, xact_decode)
// PG_RMGR(RM_SMGR_ID, "Storage", smgr_redo, smgr_desc, smgr_identify, NULL, NULL, NULL, NULL)
// PG_RMGR(RM_CLOG_ID, "CLOG", clog_redo, clog_desc, clog_identify, NULL, NULL, NULL, NULL)
// PG_RMGR(RM_DBASE_ID, "Database", dbase_redo, dbase_desc, dbase_identify, NULL, NULL, NULL, NULL)
// PG_RMGR(RM_TBLSPC_ID, "Tablespace", tblspc_redo, tblspc_desc, tblspc_identify, NULL, NULL, NULL, NULL)
// PG_RMGR(RM_MULTIXACT_ID, "MultiXact", multixact_redo, multixact_desc, multixact_identify, NULL, NULL, NULL, NULL)
// PG_RMGR(RM_RELMAP_ID, "RelMap", relmap_redo, relmap_desc, relmap_identify, NULL, NULL, NULL, NULL)
// PG_RMGR(RM_STANDBY_ID, "Standby", standby_redo, standby_desc, standby_identify, NULL, NULL, NULL, standby_decode)
// PG_RMGR(RM_HEAP2_ID, "Heap2", heap2_redo, heap2_desc, heap2_identify, NULL, NULL, heap_mask, heap2_decode)
// PG_RMGR(RM_HEAP_ID, "Heap", heap_redo, heap_desc, heap_identify, NULL, NULL, heap_mask, heap_decode)
// PG_RMGR(RM_BTREE_ID, "Btree", btree_redo, btree_desc, btree_identify, btree_xlog_startup, btree_xlog_cleanup, btree_mask, NULL)
// PG_RMGR(RM_HASH_ID, "Hash", hash_redo, hash_desc, hash_identify, NULL, NULL, hash_mask, NULL)
// PG_RMGR(RM_GIN_ID, "Gin", gin_redo, gin_desc, gin_identify, gin_xlog_startup, gin_xlog_cleanup, gin_mask, NULL)
// PG_RMGR(RM_GIST_ID, "Gist", gist_redo, gist_desc, gist_identify, gist_xlog_startup, gist_xlog_cleanup, gist_mask, NULL)
// PG_RMGR(RM_SEQ_ID, "Sequence", seq_redo, seq_desc, seq_identify, NULL, NULL, seq_mask, NULL)
// PG_RMGR(RM_SPGIST_ID, "SPGist", spg_redo, spg_desc, spg_identify, spg_xlog_startup, spg_xlog_cleanup, spg_mask, NULL)
// PG_RMGR(RM_BRIN_ID, "BRIN", brin_redo, brin_desc, brin_identify, NULL, NULL, brin_mask, NULL)
// PG_RMGR(RM_COMMIT_TS_ID, "CommitTs", commit_ts_redo, commit_ts_desc, commit_ts_identify, NULL, NULL, NULL, NULL)
// PG_RMGR(RM_REPLORIGIN_ID, "ReplicationOrigin", replorigin_redo, replorigin_desc, replorigin_identify, NULL, NULL, NULL, NULL)
// PG_RMGR(RM_GENERIC_ID, "Generic", generic_redo, generic_desc, generic_identify, NULL, NULL, generic_mask, NULL)
// PG_RMGR(RM_LOGICALMSG_ID, "LogicalMessage", logicalmsg_redo, logicalmsg_desc, logicalmsg_identify, NULL, NULL, NULL, logicalmsg_decode)

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
