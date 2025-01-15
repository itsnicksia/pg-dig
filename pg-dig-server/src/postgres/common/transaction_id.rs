use scroll::Pread;

#[repr(C)]
#[derive(Debug, Copy, Clone, Pread, PartialEq)]
pub struct TransactionId(pub u32);

pub const INVALID_TRANSACTION_ID: TransactionId = TransactionId(0);
pub const BOOTSTRAP_TRANSACTION_ID: TransactionId = TransactionId(1);
pub const FROZEN_TRANSACTION_ID: TransactionId = TransactionId(2);
pub const FIRST_NORMAL_TRANSACTION_ID: TransactionId = TransactionId(3);
pub const MAX_TRANSACTION_ID: u32 = 0xFFFFFFFF;
