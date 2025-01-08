#[derive(Debug)]
pub struct Info {
    pub block_number: u32,
    pub(crate) table_name: String,
    pub(crate) fork_name: String,
}