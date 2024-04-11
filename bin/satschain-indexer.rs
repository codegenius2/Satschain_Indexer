use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use futures::future::join_all;
use log::*;
use satschain_indexer::{
    configs::Config,
    db::{BlockFetchedData, Database, DatabaseTables},
    explorer,
    genesis::get_genesis_allocations,
    rpc::Rpc,
};
use simple_logger::SimpleLogger;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main()]
async fn main() -> std::io::Result<()> {
    let log = SimpleLogger::new().with_level(LevelFilter::Info);

    dotenv().ok();

    let explorer_server_port: String = env::var("EXPLORER_SERVER_PORT")
        .unwrap_or_else(|_| "8200".to_string());

    let config = Config::new();

    if config.debug {
        log.with_level(LevelFilter::Debug).init().unwrap();
    } else {
        log.init().unwrap();
    }

    let rpc = Rpc::new(&config).await;

    let db = Database::new(
        config.db_host.clone(),
        config.db_username.clone(),
        config.db_password.clone(),
        config.db_name.clone(),
        config.chain.clone(),
    )
    .await;

    if config.ws_url.is_some() && config.end_block == 0
        || config.end_block == -1
    {
        tokio::spawn({
            let rpc: Rpc = rpc.clone();
            let db: Database = db.clone();

            async move {
                loop {
                    rpc.listen_blocks(&db).await;

                    sleep(Duration::from_millis(500)).await;
                }
            }
        });
    }

    let t = HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(explorer::index)) // GET request to "/"
            .route("/status", web::get().to(explorer::status)) // GET request to "/status"
            .route(
                "/api/v2/blocks",
                web::get().to(explorer::handle_get_blocks),
            )
            .route(
                "/api/v2/blocks/{id}",
                web::get().to(explorer::handle_get_block_by_id),
            )
            .route(
                "/api/v2/transactions",
                web::get().to(explorer::handle_get_transactions),
            )
            .route(
                "/api/v2/transactions/{id}",
                web::get().to(explorer::handle_get_transaction_by_id),
            )
            .route(
                "/api/eth_get_balance",
                web::get().to(explorer::handle_eth_get_balance),
            )
            .route("/api/balance", web::get().to(explorer::handle_balance))
            .route(
                "/api/balancemulti",
                web::get().to(explorer::handle_balancemulti),
            )
            .route(
                "/api/pendingtxlist",
                web::get().to(explorer::handle_pendingtxlist),
            )
            .route("/api/txlist", web::get().to(explorer::handle_txlist))
            .route(
                "/api/txlistinternal",
                web::get().to(explorer::handle_txlistinternal),
            )
            .route("/api/tokentx", web::get().to(explorer::handle_tokentx))
            .route(
                "/api/tokenbalance",
                web::get().to(explorer::handle_tokenbalance),
            )
            .route(
                "/api/getblockreward",
                web::get().to(explorer::handle_getblockreward),
            )
            .route(
                "/api/getblockcountdown",
                web::get().to(explorer::handle_getblockcountdown),
            )
            .route(
                "/api/getblocknobytime",
                web::get().to(explorer::handle_getblocknobytime),
            )
            .route(
                "/api/eth_block_number",
                web::get().to(explorer::handle_eth_block_number),
            )
            .route(
                "/api/listcontracts",
                web::get().to(explorer::handle_listcontracts),
            )
            .route("/api/getabi", web::get().to(explorer::handle_getabi))
            .route(
                "/api/getsourcecode",
                web::get().to(explorer::handle_getsourcecode),
            )
            .route(
                "/api/getcontractcreation",
                web::get().to(explorer::handle_getcontractcreation),
            )
            .route("/api/verify", web::get().to(explorer::handle_verify))
            .route(
                "/api/verify_via_sourcify",
                web::get().to(explorer::handle_verify_via_sourcify),
            )
            .route(
                "/api/verify_vyper_contract",
                web::get().to(explorer::handle_verify_vyper_contract),
            )
            .route(
                "/api/verifysourcecode",
                web::get().to(explorer::handle_verifysourcecode),
            )
            .route(
                "/api/checkverifystatus",
                web::get().to(explorer::handle_checkverifystatus),
            )
            .route(
                "/api/verifyproxycontract",
                web::get().to(explorer::handle_verifyproxycontract),
            )
            .route(
                "/api/checkproxyverification",
                web::get().to(explorer::handle_checkproxyverification),
            )
            .route(
                "/api/get_logs",
                web::get().to(explorer::handle_get_logs),
            )
            .route(
                "/api/token_supply",
                web::get().to(explorer::handle_token_supply),
            )
            .route(
                "/api/get_token",
                web::get().to(explorer::handle_get_token),
            )
            .route(
                "/api/get_token_holders",
                web::get().to(explorer::handle_get_token_holders),
            )
            .route(
                "/api/bridged_token_list",
                web::get().to(explorer::handle_bridged_token_list),
            )
            .route(
                "/api/get_tx_info",
                web::get().to(explorer::handle_get_tx_info),
            )
            .route(
                "/api/get_tx_receipt_status",
                web::get().to(explorer::handle_get_tx_receipt_status),
            )
            .route(
                "/api/get_status",
                web::get().to(explorer::handle_get_status),
            )
    })
    .bind(format!("localhost:{}", explorer_server_port))? // Bind server to localhost:8080
    .run();
    tokio::spawn(t);

    loop {
        if !config.new_blocks_only {
            sync_chain(&rpc, &db, &config).await;
        }
        sleep(Duration::from_secs(30)).await;
    }
}

async fn sync_chain(rpc: &Rpc, db: &Database, config: &Config) {
    info!("here");
    let mut indexed_blocks = db.get_indexed_blocks().await;

    // If there are no indexed blocks, insert the genesis transactions
    if indexed_blocks.is_empty() {
        let genesis_transactions =
            get_genesis_allocations(config.chain.clone());
        db.store_items(
            &genesis_transactions,
            DatabaseTables::Transactions.as_str(),
        )
        .await;
    }

    let last_block = if config.end_block != 0 {
        config.end_block as u32
    } else {
        rpc.get_last_block().await
    };

    let full_block_range: Vec<u32> =
        (config.start_block..last_block).collect();

    let missing_blocks: Vec<&u32> = full_block_range
        .iter()
        .filter(|block| !indexed_blocks.contains(block))
        .collect();

    let total_missing_blocks = missing_blocks.len();

    // If the program uses a block range and finishes shutdown gracefully
    if config.end_block != 0 && total_missing_blocks == 0 {
        std::process::exit(0);
    }

    let missing_blocks_chunks = missing_blocks.chunks(config.batch_size);

    for missing_blocks_chunk in missing_blocks_chunks {
        let mut work = vec![];

        for block_number in missing_blocks_chunk {
            work.push(rpc.fetch_block(block_number, &config.chain))
        }

        let results = join_all(work).await;

        let mut fetched_data = BlockFetchedData {
            blocks: Vec::new(),
            contracts: Vec::new(),
            logs: Vec::new(),
            traces: Vec::new(),
            transactions: Vec::new(),
            withdrawals: Vec::new(),
            erc20_transfers: Vec::new(),
            erc721_transfers: Vec::new(),
            erc1155_transfers: Vec::new(),
            dex_trades: Vec::new(),
        };

        for result in results {
            match result {
                Some((
                    mut blocks,
                    mut transactions,
                    mut logs,
                    mut contracts,
                    mut traces,
                    mut withdrawals,
                    mut erc20_transfers,
                    mut erc721_transfers,
                    mut erc1155_transfers,
                    mut dex_trades,
                )) => {
                    fetched_data.blocks.append(&mut blocks);
                    fetched_data.transactions.append(&mut transactions);
                    fetched_data.logs.append(&mut logs);
                    fetched_data.contracts.append(&mut contracts);
                    fetched_data.traces.append(&mut traces);
                    fetched_data.withdrawals.append(&mut withdrawals);
                    fetched_data
                        .erc20_transfers
                        .append(&mut erc20_transfers);
                    fetched_data
                        .erc721_transfers
                        .append(&mut erc721_transfers);
                    fetched_data
                        .erc1155_transfers
                        .append(&mut erc1155_transfers);
                    fetched_data.dex_trades.append(&mut dex_trades);
                }
                None => continue,
            }
        }

        db.store_data(&fetched_data).await;

        for block in fetched_data.blocks.iter() {
            info!("block_number {}", block.clone().number);
            indexed_blocks.insert(block.number);
        }
    }
}
