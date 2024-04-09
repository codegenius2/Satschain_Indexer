use crate::configs::Config;
use crate::db::models::block::DatabaseBlock;
use crate::db::Database;
use actix_web::{web, web::Json, HttpResponse, Responder};
use ethers::etherscan::blocks;
use serde::{Deserialize, Serialize};

// Common Structs for Account Module

#[derive(Deserialize, Serialize)]
pub struct Miner {
    ens_domain_name: Option<String>,
    hash: String,
    implementation_name: Option<String>,
    // Add other fields if there are more inside `miner` not listed here
}

#[derive(Deserialize, Serialize)]
pub struct Reward {
    reward: String,
    type_field: String, // "type" is a reserved keyword in Rust, hence "type_field"
}
#[derive(Deserialize, Serialize)]
pub struct BlockResponse {
    base_fee_per_gas: String,
    blob_gas_used: String,
    blob_tx_count: u32,
    burnt_fees: String,
    burnt_fees_percentage: f64,
    difficulty: String,
    excess_blob_gas: String,
    gas_limit: String,
    gas_target_percentage: f64,
    gas_used: String,
    gas_used_percentage: f64,
    hash: String,
    height: u64,
    miner: Miner,
    nonce: String,
    parent_hash: String,
    priority_fee: u64,
    rewards: Vec<Reward>,
    size: u64,
    timestamp: String,
    total_difficulty: String,
    tx_count: u32,
    tx_fees: String,
    r#type: String,
    uncles_hashes: Vec<String>,
    withdrawals_count: Option<u32>, // Assuming nullable means Option in Rust
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
            timestamp: db_block.timestamp.to_string(),
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

#[derive(Deserialize)]
pub struct GetBlockQuery {}

#[derive(Deserialize)]
pub struct AccountQuery {
    address: String,
}

#[derive(Deserialize)]
pub struct MultiAccountQuery {
    address: String, // Expected to contain multiple addresses separated by commas
}

#[derive(Deserialize)]
pub struct TransactionQuery {
    address: String,
    startblock: Option<i64>,
    endblock: Option<i64>,
    page: Option<i32>,
    offset: Option<i32>,
    sort: Option<String>,
}

#[derive(Deserialize)]
pub struct TxListInternalQuery {
    txhash: String,
    startblock: Option<i64>,
    endblock: Option<i64>,
    page: Option<i32>,
    offset: Option<i32>,
    sort: Option<String>,
}

#[derive(Deserialize)]
pub struct TokenTxQuery {
    address: String,
    page: Option<i32>,
    offset: Option<i32>,
    sort: Option<String>,
}

#[derive(Deserialize)]
pub struct TokenBalanceQuery {
    contractaddress: String,
    address: String,
}

// Common Struct for Block Module
#[derive(Deserialize)]
pub struct BlockQuery {
    blockno: Option<i64>,
    timestamp: Option<i64>,
    closest: Option<String>, // "before" or "after"
}

#[derive(Deserialize)]
pub struct ContractQuery {
    address: String,
}

#[derive(Deserialize)]
pub struct ContractVerifyQuery {
    addressHash: String,
    name: Option<String>,
    compilerVersion: Option<String>,
    optimization: Option<bool>,
    contractSourceCode: Option<String>,
}

#[derive(Deserialize)]
pub struct ContractCreationQuery {
    contractaddresses: String, // Expected to contain multiple addresses separated by commas
}

#[derive(Deserialize)]
pub struct SimpleQuery {
    addressHash: String,
}

#[derive(Deserialize)]
pub struct SourceCodeQuery {
    addressHash: String,
    codeformat: Option<String>,
    contractaddress: Option<String>,
    contractname: Option<String>,
    compilerversion: Option<String>,
    sourceCode: Option<String>,
}

#[derive(Deserialize)]
pub struct VerifyStatusQuery {
    guid: String,
}

// Struct for Log Queries
#[derive(Deserialize)]
pub struct LogQuery {
    fromBlock: i64,
    toBlock: i64,
    address: String,
    topic0: String,
    topic1: Option<String>,
    topic0_1_opr: Option<String>, // Operator between topic0 and topic1 if applicable
}

// Struct for Token and Stats Queries
#[derive(Deserialize)]
pub struct TokenQuery {
    contractaddress: String,
}

// Struct for Token Holders Pagination
#[derive(Deserialize)]
pub struct TokenHoldersQuery {
    contractaddress: String,
    page: i32,
    offset: i32,
}

// Struct for Bridged Token List Query
#[derive(Deserialize)]
pub struct BridgedTokenListQuery {
    chainid: i32,
    page: i32,
    offset: i32,
}

// Struct for Transaction Queries
#[derive(Deserialize)]
pub struct TransactionHashQuery {
    txhash: String,
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the EVM Indexer API!")
}

pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("Status: Running")
}

// Handler Functions
pub async fn handle_getblocks(
    query: web::Query<GetBlockQuery>,
) -> impl Responder {
    let config = Config::new();

    let db = Database::new(
        config.db_host.clone(),
        config.db_username.clone(),
        config.db_password.clone(),
        config.db_name.clone(),
        config.chain.clone(),
    )
    .await;

    let database_blocks = db.get_blocks().await;
    let blocks: Vec<BlockResponse> =
        database_blocks.into_iter().map(BlockResponse::from).collect();

    // println!("{:?}", blocks);
    HttpResponse::Ok().content_type("application/json").json(blocks)
}

pub async fn handle_eth_get_balance(
    query: web::Query<AccountQuery>,
) -> impl Responder {
    HttpResponse::Ok()
        .json(format!("Fetching ETH balance for {}", query.address))
}

pub async fn handle_balance(
    query: web::Query<AccountQuery>,
) -> impl Responder {
    HttpResponse::Ok()
        .json(format!("Fetching balance for {}", query.address))
}

pub async fn handle_balancemulti(
    query: web::Query<MultiAccountQuery>,
) -> impl Responder {
    HttpResponse::Ok()
        .json(format!("Fetching balances for {}", query.address))
}

pub async fn handle_pendingtxlist(
    query: web::Query<TransactionQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Fetching pending transactions for {}",
        query.address
    ))
}

pub async fn handle_txlist(
    query: web::Query<TransactionQuery>,
) -> impl Responder {
    HttpResponse::Ok()
        .json(format!("Fetching transactions for {}", query.address))
}

pub async fn handle_txlistinternal(
    query: web::Query<TxListInternalQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Fetching internal transactions for transaction hash {}",
        query.txhash
    ))
}

pub async fn handle_tokentx(
    query: web::Query<TokenTxQuery>,
) -> impl Responder {
    HttpResponse::Ok()
        .json(format!("Fetching token transactions for {}", query.address))
}

pub async fn handle_tokenbalance(
    query: web::Query<TokenBalanceQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Fetching token balance from contract {} for {}",
        query.contractaddress, query.address
    ))
}

pub async fn handle_getblockreward(
    query: web::Query<BlockQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Fetching block reward for block number {}",
        query.blockno.unwrap_or_default()
    ))
}

pub async fn handle_getblockcountdown(
    query: web::Query<BlockQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Fetching block countdown for block number {}",
        query.blockno.unwrap_or_default()
    ))
}

pub async fn handle_getblocknobytime(
    query: web::Query<BlockQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Fetching block number by time {}, closest {}",
        query.timestamp.unwrap_or_default(),
        query.closest.as_ref().unwrap_or(&"N/A".to_string())
    ))
}

pub async fn handle_eth_block_number() -> impl Responder {
    HttpResponse::Ok().json("Fetching current Ethereum block number")
}

// Handler functions for Contract Module
pub async fn handle_listcontracts() -> impl Responder {
    HttpResponse::Ok().json("Listing all contracts")
}

pub async fn handle_getabi(
    query: web::Query<SimpleQuery>,
) -> impl Responder {
    HttpResponse::Ok()
        .json(format!("Fetching ABI for address {}", query.addressHash))
}

pub async fn handle_getsourcecode(
    query: web::Query<SimpleQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Fetching source code for address {}",
        query.addressHash
    ))
}

pub async fn handle_getcontractcreation(
    query: web::Query<ContractCreationQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Fetching contract creation info for addresses {}",
        query.contractaddresses
    ))
}

pub async fn handle_verify(
    query: web::Query<ContractVerifyQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Verifying contract at address {} with compiler version ",
        query.addressHash // query.compilerVersion.unwrap_or_default()
    ))
}

pub async fn handle_verify_via_sourcify(
    query: web::Query<SimpleQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Verifying contract via Sourcify for address {}",
        query.addressHash
    ))
}

pub async fn handle_verify_vyper_contract(
    query: web::Query<ContractVerifyQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Verifying Vyper contract at address {}",
        query.addressHash
    ))
}

pub async fn handle_verifysourcecode(
    query: web::Query<SourceCodeQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Verifying source code for address {}, format ",
        query.addressHash // query.codeformat.unwrap_or_default()
    ))
}

pub async fn handle_checkverifystatus(
    query: web::Query<VerifyStatusQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Checking verification status with GUID {}",
        query.guid
    ))
}

pub async fn handle_verifyproxycontract(
    query: web::Query<SimpleQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Verifying proxy contract for address {}",
        query.addressHash
    ))
}

pub async fn handle_checkproxyverification(
    query: web::Query<VerifyStatusQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Checking proxy verification status with GUID {}",
        query.guid
    ))
}

// Handler functions for Logs Module
pub async fn handle_get_logs(
    query: web::Query<LogQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!("Fetching logs from block {} to {} for address {} with topics {} and", 
        query.fromBlock, query.toBlock, query.address, query.topic0)) //, query.topic1.unwrap_or_default()
}

// Handler functions for Stats Module
pub async fn handle_token_supply(
    query: web::Query<TokenQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Fetching token supply for contract address {}",
        query.contractaddress
    ))
}

// Handler functions for Token Module
pub async fn handle_get_token(
    query: web::Query<TokenQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Fetching token details for contract address {}",
        query.contractaddress
    ))
}

pub async fn handle_get_token_holders(
    query: web::Query<TokenHoldersQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!("Fetching token holders for contract address {} on page {} with offset {}", 
        query.contractaddress, query.page, query.offset))
}

pub async fn handle_bridged_token_list(
    query: web::Query<BridgedTokenListQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!("Fetching bridged token list for chain ID {} on page {} with offset {}", 
        query.chainid, query.page, query.offset))
}

// Handler functions for Transaction Module
pub async fn handle_get_tx_info(
    query: web::Query<TransactionHashQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Fetching transaction info for hash {}",
        query.txhash
    ))
}

pub async fn handle_get_tx_receipt_status(
    query: web::Query<TransactionHashQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Fetching transaction receipt status for hash {}",
        query.txhash
    ))
}

pub async fn handle_get_status(
    query: web::Query<TransactionHashQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Fetching transaction status for hash {}",
        query.txhash
    ))
}
