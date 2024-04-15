use crate::{configs::Config, db::Database, explorer::models::*};
use actix_web::{web, HttpResponse, Responder};
use log::info;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the EVM Indexer API!")
}

pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("Status: Running")
}

// Handler Functions
pub async fn handle_get_blocks(
    query: web::Query<GetBlockQuery>,
) -> impl Responder {
    info!("Here");
    let skip_count = query.items_count.unwrap_or(0);

    let config = Config::new();

    let db = Database::new(
        config.db_host.clone(),
        config.db_username.clone(),
        config.db_password.clone(),
        config.db_name.clone(),
        config.chain.clone(),
    )
    .await;

    let database_blocks = db.get_blocks(skip_count, 50).await;
    println!("-------- db_blocks -------- {:?}", database_blocks);
    let blocks: Vec<BlockResponse> =
        database_blocks.into_iter().map(BlockResponse::from).collect();

    let block_number;
    if let Some(last_block) = blocks.last() {
        block_number = last_block.height;
    } else {
        block_number = 1;
    }
    let next_page = NextPageParams {
        block_number: block_number - 1,
        items_count: skip_count + blocks.len() as u32,
    };

    let rlt =
        BlockResponseData { items: blocks, next_page_params: next_page };
    // println!("{:?}", blocks);
    HttpResponse::Ok().content_type("application/json").json(rlt)
}

// Define a handler function that accepts a web::Path wrapping a tuple containing the ID
pub async fn handle_get_block_by_id(
    query: web::Path<(u64,)>,
) -> impl Responder {
    let block_id = query.0;
    info!("You requested information for block ID: {}", block_id.clone());
    let config = Config::new();

    let db = Database::new(
        config.db_host.clone(),
        config.db_username.clone(),
        config.db_password.clone(),
        config.db_name.clone(),
        config.chain.clone(),
    )
    .await;

    let database_block =
        BlockResponse::from(db.get_block_by_id(block_id.clone()).await);
    HttpResponse::Ok()
        .content_type("application/json")
        .json(database_block)
}

pub async fn handle_get_transactions(
    query: web::Query<GetTransactionQuery>,
) -> impl Responder {
    let skip_count = query.items_count.unwrap_or(0);

    let config = Config::new();

    let db = Database::new(
        config.db_host.clone(),
        config.db_username.clone(),
        config.db_password.clone(),
        config.db_name.clone(),
        config.chain.clone(),
    )
    .await;

    let database_transactions = db.get_transactions(skip_count, 50).await;
    println!(
        "-------- db_transactions -------- {:?}",
        database_transactions
    );
    let transactions: Vec<TransactionResponse> = database_transactions
        .into_iter()
        .map(TransactionResponse::from)
        .collect();

    let block_number;
    if let Some(last_block) = transactions.last() {
        block_number = last_block.block;
    } else {
        block_number = 1;
    }
    let next_page = NextPageParams {
        block_number: block_number - 1,
        items_count: skip_count + transactions.len() as u32,
    };

    let rlt = TransactionResponseData {
        items: transactions,
        next_page_params: next_page,
    };
    // println!("{:?}", blocks);
    HttpResponse::Ok().content_type("application/json").json(rlt)
}

// Define a handler function that accepts a web::Path wrapping a tuple containing the ID
pub async fn handle_get_transaction_by_id(
    query: web::Path<(String,)>,
) -> impl Responder {
    let hash = query.into_inner().0;
    info!("You requested information for block ID: {}", hash.clone());
    let config = Config::new();

    let db = Database::new(
        config.db_host.clone(),
        config.db_username.clone(),
        config.db_password.clone(),
        config.db_name.clone(),
        config.chain.clone(),
    )
    .await;

    let database_transaction = TransactionResponse::from(
        db.get_transaction_by_id(hash.clone()).await,
    );

    HttpResponse::Ok()
        .content_type("application/json")
        .json(database_transaction)
}

pub async fn handle_get_transaction_summary_for_id(
    query: web::Path<(String,)>,
) -> impl Responder {
    let hash = query.into_inner().0;
    // let x: TransactionSummaryResponseData{
    //     data: TransactionSummaryResponse{
    //         debug_data: TransactionDebugData{
    //             is_prompt_truncated: false,
    //             model_classification_type: "approved".to_string(),
    //             post_llm_classification_type: "approved".to_string(),
    //             summary_template: {},
    //             transaction_hash: hash  ,
    //         },
    //         summaries: vec![],
    //     },
    //     success: true
    // };
    HttpResponse::Ok().content_type("application/json").json(hash)
}
pub async fn handle_get_indexing_status(
    query: web::Query<EmptyQuery>,
) -> impl Responder {
    let x = IndexingStatusResponse {
        finished_indexing: true,
        finished_indexing_blocks: true,
        indexed_blocks_ratio: "1.00".to_string(),
        indexed_internal_transactions_ratio: "1.00".to_string(),
    };
    HttpResponse::Ok().content_type("application/json").json(x)
}
pub async fn handle_get_stats(
    query: web::Query<EmptyQuery>,
) -> impl Responder {
    info!("-------------------- You are trying to get stats --------------------");
    let config = Config::new();

    let db = Database::new(
        config.db_host.clone(),
        config.db_username.clone(),
        config.db_password.clone(),
        config.db_name.clone(),
        config.chain.clone(),
    )
    .await;

    let mut stats_response = StatsResponse::new();
    let info_for_average_block = db.get_info_for_average_block().await;
    info!("****** average_block: {:?}", info_for_average_block);
    stats_response.average_block_time =
        ((info_for_average_block.end_timestamp
            - info_for_average_block.start_timestamp) as u64
            * 1000
            / (info_for_average_block.end_number
                - info_for_average_block.start_number)
                as u64) as u32;

    stats_response.coin_image = String::from("https://assets.coingecko.com/coins/images/279/small/ethereum.png?1696501628");
    stats_response.coin_price = String::from("3504.49");
    stats_response.coin_price_change_percentage = 0.85;
    stats_response.gas_price_updated_at =
        String::from("2024-04-11T19:07:36.078590Z");
    stats_response.gas_prices = GasPricesType {
        average: GasPriceType {
            base_fee: 7.27,
            fiat_price: "2.08".to_string(),
            price: 8.29,
            priority_fee: 1.02,
            priority_fee_wei: "1018670328".to_string(),
            time: 6190.575,
            wei: "28280383240".to_string(),
        },
        fast: GasPriceType {
            base_fee: 7.27,
            fiat_price: "2.08".to_string(),
            price: 8.29,
            priority_fee: 1.02,
            priority_fee_wei: "1018670328".to_string(),
            time: 6190.575,
            wei: "28280383240".to_string(),
        },
        slow: GasPriceType {
            base_fee: 7.27,
            fiat_price: "2.08".to_string(),
            price: 8.29,
            priority_fee: 1.02,
            priority_fee_wei: "1018670328".to_string(),
            time: 6190.575,
            wei: "28280383240".to_string(),
        },
    };
    stats_response.gas_prices_update_in = 29909;
    stats_response.gas_used_today = String::from("108484462870");
    stats_response.market_cap = String::from("420786039413.37880565");
    stats_response.network_utilization_percentage = 53.92967968668905;
    stats_response.secondary_coin_price = None;
    stats_response.static_gas_price = None;
    stats_response.total_addresses = "327784281".to_string();
    stats_response.total_blocks =
        info_for_average_block.end_number.to_string();
    stats_response.total_gas_used = "0".to_string();
    stats_response.total_transactions = "2331571580".to_string();
    stats_response.transactions_today = "1207828".to_string();
    stats_response.tvl = None;

    info!(
        "****** average_calcuation: {:?}",
        (info_for_average_block.end_timestamp
            - info_for_average_block.start_timestamp)
            / (info_for_average_block.end_number
                - info_for_average_block.start_number)
    );

    HttpResponse::Ok()
        .content_type("application/json")
        .json(stats_response)
}

pub async fn handle_main_page_blocks(
    query: web::Query<EmptyQuery>,
) -> impl Responder {
    let config = Config::new();

    info!(" ((((((((((( We are here!!! )))))))))))");
    let db = Database::new(
        config.db_host.clone(),
        config.db_username.clone(),
        config.db_password.clone(),
        config.db_name.clone(),
        config.chain.clone(),
    )
    .await;

    let database_blocks = db.get_blocks(0, 6).await;
    info!("-------- main_page db_blocks -------- {:?}", database_blocks);
    let blocks: Vec<BlockResponse> =
        database_blocks.into_iter().map(BlockResponse::from).collect();

    HttpResponse::Ok().content_type("application/json").json(blocks)
}

pub async fn handle_main_page_transactions(
    query: web::Query<EmptyQuery>,
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

    let database_transactions = db.get_transactions(0, 6).await;
    let transactions: Vec<TransactionResponse> = database_transactions
        .into_iter()
        .map(TransactionResponse::from)
        .collect();
    info!(
        "-------- main_page_db_transactions -------- {:?}",
        transactions
    );

    HttpResponse::Ok().content_type("application/json").json(transactions)
}

pub async fn handle_get_stats_charts_transactions(
    query: web::Query<EmptyQuery>,
) -> impl Responder {
    info!("-------------------- You are trying to get stats charts transactions --------------------");
    let config = Config::new();

    let db = Database::new(
        config.db_host.clone(),
        config.db_username.clone(),
        config.db_password.clone(),
        config.db_name.clone(),
        config.chain.clone(),
    )
    .await;

    let chart_transaction = db.get_chart_transaction_data().await;
    let chart_transaction_data =
        ChartTransactionResponseData { chart_data: chart_transaction };

    info!("****** chart_transaction_data: {:?}", chart_transaction_data);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(chart_transaction_data)
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
