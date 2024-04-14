use clickhouse::Row;
use ethers::types::Log;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::utils::format::{format_address, format_bytes, format_hash};

#[serde_as]
#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct DatabaseInfoForSync {
    pub end_block: u32,
    pub missing_blocks: Vec<u32>,
    pub timestamp: u32,
}
