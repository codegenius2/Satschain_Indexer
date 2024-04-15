use crate::db::models::{
    block::DatabaseBlock,
    transaction::{DatabaseTransaction, TransactionStatus},
};

use chrono::{TimeZone, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Deserialize, Serialize, Clone)]
pub struct Miner {
    pub ens_domain_name: Option<String>,
    pub hash: String,
    pub implementation_name: Option<String>,
    // Add other fields if there are more inside `miner` not listed here
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Reward {
    pub reward: String,
    pub type_field: String, // "type" is a reserved keyword in Rust, hence "type_field"
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BlockResponse {
    pub base_fee_per_gas: String,
    pub blob_gas_used: String,
    pub blob_tx_count: u32,
    pub burnt_fees: String,
    pub burnt_fees_percentage: f64,
    pub difficulty: String,
    pub excess_blob_gas: String,
    pub gas_limit: String,
    pub gas_target_percentage: f64,
    pub gas_used: String,
    pub gas_used_percentage: f64,
    pub hash: String,
    pub height: u64,
    pub miner: Miner,
    pub nonce: String,
    pub parent_hash: String,
    pub priority_fee: u64,
    pub rewards: Vec<Reward>,
    pub size: u64,
    pub timestamp: String,
    pub total_difficulty: String,
    pub tx_count: u32,
    pub tx_fees: String,
    pub r#type: String,
    pub uncles_hashes: Vec<String>,
    pub withdrawals_count: Option<u32>,
}

impl From<DatabaseBlock> for BlockResponse {
    fn from(db_block: DatabaseBlock) -> Self {
        BlockResponse {
            base_fee_per_gas: db_block
                .base_fee_per_gas
                .map(|v| v.to_string())
                .unwrap_or_default(),
            blob_gas_used: db_block.gas_used.to_string(),
            blob_tx_count: db_block.transactions as u32,
            burnt_fees: db_block.burned.to_string(),
            burnt_fees_percentage: 0.0, // Calculate or provide a default
            difficulty: db_block.difficulty.to_string(),
            excess_blob_gas: "0".to_string(), // Default if not available
            gas_limit: db_block.gas_limit.to_string(),
            gas_target_percentage: 0.0, // Calculate or provide a default
            gas_used: db_block.gas_used.to_string(),
            gas_used_percentage: 0.0, // Calculate based on gas_limit and gas_used
            hash: db_block.hash,
            height: db_block.number as u64,
            miner: Miner {
                ens_domain_name: None,
                hash: db_block.miner,
                implementation_name: None,
            },
            nonce: db_block.nonce,
            parent_hash: db_block.parent_hash,
            priority_fee: 0, // Default if not available
            rewards: vec![], // Construct Reward vector as necessary
            size: db_block.size as u64,
            timestamp: Utc
                .timestamp(db_block.timestamp as i64, 0)
                .format("%Y-%m-%dT%H:%M:%S%.fZ")
                .to_string(),
            total_difficulty: db_block
                .total_difficulty
                .map(|v| v.to_string())
                .unwrap_or_default(),
            tx_count: db_block.transactions as u32,
            tx_fees: db_block.total_fee_reward.to_string(),
            r#type: "block".to_string(),
            uncles_hashes: db_block.uncles,
            withdrawals_count: None, // Default or convert if possible
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FeeType {
    pub r#type: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransAccountType {
    pub ens_domain_name: String,
    pub hash: String,
    pub implementation_name: Option<String>,
    pub is_contract: bool,
    pub is_verified: Option<bool>,
    pub metadata: Option<String>,
    pub name: Option<String>,
    pub private_tags: Vec<String>,
    public_tags: Vec<String>,
    pub watchlist_names: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransactionResponse {
    pub actions: Vec<String>,
    pub base_fee_per_gas: String,
    pub block: u64,
    pub confirmation_duration: Vec<u32>,
    pub confirmations: u32,
    pub created_contract: Option<String>,
    pub decode_input: Option<String>,
    pub exchange_rate: String,
    pub fee: FeeType,
    pub from: TransAccountType,
    pub gas_limit: String,
    pub gas_price: String,
    pub gas_used: String,
    pub has_error_in_internal_txs: bool,
    pub hash: String,
    pub max_fee_per_gas: String,
    pub max_priority_fee_per_gas: String,
    pub method: Option<String>,
    pub nonce: u32,
    pub position: u32,
    pub priority_fee: Option<String>,
    pub raw_input: String,
    pub result: String,
    pub revert_reason: Option<String>,
    pub status: String,
    pub timestamp: String,
    pub to: TransAccountType,
    pub token_transfers: Option<String>,
    pub token_transfers_overflow: Option<String>,
    pub tx_burnt_fee: Option<String>,
    pub tx_tag: Option<String>,
    pub tx_types: Vec<String>,
    pub r#type: u32,
    pub value: String,
}

impl From<DatabaseTransaction> for TransactionResponse {
    fn from(dt: DatabaseTransaction) -> Self {
        TransactionResponse {
            actions: Vec::new(),
            base_fee_per_gas: dt
                .base_fee_per_gas
                .map_or(String::new(), |v| v.to_string()),
            block: dt.block_number as u64,
            confirmation_duration: Vec::new(),
            confirmations: 0,
            created_contract: dt.contract_created,
            decode_input: Some(dt.input.clone()),
            exchange_rate: String::new(),
            fee: FeeType {
                r#type: String::from("Standard"),
                value: dt
                    .effective_transaction_fee
                    .map_or(String::new(), |v| v.to_string()),
            },
            from: TransAccountType {
                ens_domain_name: String::new(),
                hash: dt.from,
                implementation_name: None,
                is_contract: false,
                is_verified: None,
                metadata: None,
                name: None,
                private_tags: Vec::new(),
                public_tags: Vec::new(),
                watchlist_names: Vec::new(),
            },
            gas_limit: dt.gas.to_string(),
            gas_price: dt
                .gas_price
                .map_or(String::new(), |v| v.to_string()),
            gas_used: dt.gas_used.map_or(String::new(), |v| v.to_string()),
            has_error_in_internal_txs: false,
            hash: dt.hash,
            max_fee_per_gas: dt
                .max_fee_per_gas
                .map_or(String::new(), |v| v.to_string()),
            max_priority_fee_per_gas: dt
                .max_priority_fee_per_gas
                .map_or(String::new(), |v| v.to_string()),
            method: Some(dt.method.clone()),
            nonce: dt.nonce,
            position: dt.transaction_index as u32,
            priority_fee: None,
            raw_input: dt.input,
            result: "success".to_string(),
            revert_reason: None,
            status: match dt.status {
                Some(TransactionStatus::Success) => String::from("ok"),
                Some(TransactionStatus::Failure) => String::from("error"),
                _ => String::from("Unknown"),
            },
            timestamp: Utc
                .timestamp(dt.timestamp as i64, 0)
                .format("%Y-%m-%dT%H:%M:%S%.fZ")
                .to_string(),
            to: TransAccountType {
                ens_domain_name: String::new(),
                hash: dt.to,
                implementation_name: None,
                is_contract: false,
                is_verified: None,
                metadata: None,
                name: None,
                private_tags: Vec::new(),
                public_tags: Vec::new(),
                watchlist_names: Vec::new(),
            },
            token_transfers: None,
            token_transfers_overflow: None,
            tx_burnt_fee: dt.burned.map_or(None, |v| Some(v.to_string())),
            tx_tag: None,
            tx_types: vec![
                "coin_transfer".to_string(),
                "contract_call".to_string(),
                "token_transfer".to_string(),
            ],
            r#type: dt.transaction_type as u32,
            value: dt.value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct GasPriceType {
    pub base_fee: f64,
    pub fiat_price: String,
    pub price: f64,
    pub priority_fee: f64,
    pub priority_fee_wei: String,
    pub time: f64,
    pub wei: String,
}

#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct GasPricesType {
    pub average: GasPriceType,
    pub fast: GasPriceType,
    pub slow: GasPriceType,
}

#[serde_as]
#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct StatsResponse {
    pub average_block_time: u32,
    pub coin_image: String,
    pub coin_price: String,
    pub coin_price_change_percentage: f64,
    pub gas_price_updated_at: String,
    pub gas_prices: GasPricesType,
    pub gas_prices_update_in: u32,
    pub gas_used_today: String,
    pub market_cap: String,
    pub network_utilization_percentage: f64,
    pub secondary_coin_price: Option<f64>,
    pub static_gas_price: Option<f64>,
    pub total_addresses: String,
    pub total_blocks: String,
    pub total_gas_used: String,
    pub total_transactions: String,
    pub transactions_today: String,
    pub tvl: Option<u64>,
}

impl Default for StatsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl StatsResponse {
    pub fn new() -> Self {
        Self {
            average_block_time: 0,
            coin_image: "".to_string(),
            coin_price: "".to_string(),
            coin_price_change_percentage: 0.0,
            gas_price_updated_at: "".to_string(),
            gas_prices: GasPricesType {
                average: GasPriceType {
                    base_fee: 0.0,
                    fiat_price: "".to_string(),
                    price: 0.0,
                    priority_fee: 0.0,
                    priority_fee_wei: "".to_string(),
                    time: 0.0,
                    wei: "".to_string(),
                },
                fast: GasPriceType {
                    base_fee: 0.0,
                    fiat_price: "".to_string(),
                    price: 0.0,
                    priority_fee: 0.0,
                    priority_fee_wei: "".to_string(),
                    time: 0.0,
                    wei: "".to_string(),
                },
                slow: GasPriceType {
                    base_fee: 0.0,
                    fiat_price: "".to_string(),
                    price: 0.0,
                    priority_fee: 0.0,
                    priority_fee_wei: "".to_string(),
                    time: 0.0,
                    wei: "".to_string(),
                },
            },
            gas_prices_update_in: 0,
            gas_used_today: "".to_string(),
            market_cap: "".to_string(),
            network_utilization_percentage: 0.0,
            secondary_coin_price: None,
            static_gas_price: None,
            total_addresses: "".to_string(),
            total_blocks: "".to_string(),
            total_gas_used: "".to_string(),
            total_transactions: "".to_string(),
            transactions_today: "".to_string(),
            tvl: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct NextPageParams {
    pub block_number: u64,
    pub items_count: u32,
}

#[derive(Deserialize, Serialize)]
pub struct BlockResponseData {
    pub items: Vec<BlockResponse>,
    pub next_page_params: NextPageParams,
}

#[derive(Deserialize, Serialize)]
pub struct TransactionResponseData {
    pub items: Vec<TransactionResponse>,
    pub next_page_params: NextPageParams,
}

#[serde_as]
#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct InfoForAverageBlock {
    pub start_timestamp: u32,
    pub end_timestamp: u32,
    pub start_number: u32,
    pub end_number: u32,
}

impl Default for InfoForAverageBlock {
    fn default() -> Self {
        Self::new()
    }
}
impl InfoForAverageBlock {
    pub fn new() -> Self {
        Self {
            start_timestamp: 0,
            end_timestamp: 1,
            start_number: 0,
            end_number: 1,
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct ChartTransactionResponse {
    pub date: String,
    pub tx_count: u32,
}

#[serde_as]
#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct ChartTransactionResponseData {
    pub chart_data: Vec<ChartTransactionResponse>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SummaryTemplate {
    approved: Option<u32>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransactionDebugData {
    pub is_prompt_truncated: bool,
    pub model_classification_type: String,
    pub post_llm_classification_type: String,
    pub summary_template: SummaryTemplate,
    pub transaction_hash: String,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransactionSummaryResponse {
    pub debug_data: TransactionDebugData,
    pub summaries: Vec<u32>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransactionSummaryResponseData {
    pub data: TransactionSummaryResponse,
    pub success: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct IndexingStatusResponse {
    pub finished_indexing: bool,
    pub finished_indexing_blocks: bool,
    pub indexed_blocks_ratio: String,
    pub indexed_internal_transactions_ratio: String,
}
#[derive(Deserialize)]
pub struct GetBlockQuery {
    pub block_number: Option<u64>,
    pub items_count: Option<u32>,
}

#[derive(Deserialize)]
pub struct GetTransactionQuery {
    pub hash: Option<String>,
    pub index: Option<u32>,
    pub items_count: Option<u32>,
}

#[derive(Deserialize)]
pub struct EmptyQuery {}

#[derive(Deserialize)]
pub struct AccountQuery {
    pub address: String,
}

#[derive(Deserialize)]
pub struct MultiAccountQuery {
    pub address: String, // Expected to contain multiple addresses separated by commas
}

#[derive(Deserialize)]
pub struct TransactionQuery {
    pub address: String,
    pub startblock: Option<i64>,
    pub endblock: Option<i64>,
    pub page: Option<i32>,
    pub offset: Option<i32>,
    pub sort: Option<String>,
}

#[derive(Deserialize)]
pub struct TxListInternalQuery {
    pub txhash: String,
    pub startblock: Option<i64>,
    pub endblock: Option<i64>,
    pub page: Option<i32>,
    pub offset: Option<i32>,
    pub sort: Option<String>,
}

#[derive(Deserialize)]
pub struct TokenTxQuery {
    pub address: String,
    pub page: Option<i32>,
    pub offset: Option<i32>,
    pub sort: Option<String>,
}

#[derive(Deserialize)]
pub struct TokenBalanceQuery {
    pub contractaddress: String,
    pub address: String,
}

// Common Struct for Block Module
#[derive(Deserialize)]
pub struct BlockQuery {
    pub blockno: Option<i64>,
    pub timestamp: Option<i64>,
    pub closest: Option<String>, // "before" or "after"
}

#[derive(Deserialize)]
pub struct ContractQuery {
    pub address: String,
}

#[derive(Deserialize)]
pub struct ContractVerifyQuery {
    pub addressHash: String,
    pub name: Option<String>,
    pub compilerVersion: Option<String>,
    pub optimization: Option<bool>,
    pub contractSourceCode: Option<String>,
}

#[derive(Deserialize)]
pub struct ContractCreationQuery {
    pub contractaddresses: String, // Expected to contain multiple addresses separated by commas
}

#[derive(Deserialize)]
pub struct SimpleQuery {
    pub addressHash: String,
}

#[derive(Deserialize)]
pub struct SourceCodeQuery {
    pub addressHash: String,
    pub codeformat: Option<String>,
    pub contractaddress: Option<String>,
    pub contractname: Option<String>,
    pub compilerversion: Option<String>,
    pub sourceCode: Option<String>,
}

#[derive(Deserialize)]
pub struct VerifyStatusQuery {
    pub guid: String,
}

// Struct for Log Queries
#[derive(Deserialize)]
pub struct LogQuery {
    pub fromBlock: i64,
    pub toBlock: i64,
    pub address: String,
    pub topic0: String,
    pub topic1: Option<String>,
    pub topic0_1_opr: Option<String>, // Operator between topic0 and topic1 if applicable
}

// Struct for Token and Stats Queries
#[derive(Deserialize)]
pub struct TokenQuery {
    pub contractaddress: String,
}

// Struct for Token Holders Pagination
#[derive(Deserialize)]
pub struct TokenHoldersQuery {
    pub contractaddress: String,
    pub page: i32,
    pub offset: i32,
}

// Struct for Bridged Token List Query
#[derive(Deserialize)]
pub struct BridgedTokenListQuery {
    pub chainid: i32,
    pub page: i32,
    pub offset: i32,
}

// Struct for Transaction Queries
#[derive(Deserialize)]
pub struct TransactionHashQuery {
    pub txhash: String,
}
