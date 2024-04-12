use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use log::*;
use satschain_indexer::{
    configs::Config,
    db::Database,
    explorer::routes::configure,
    rpc::{sync_chain, Rpc},
};
use simple_logger::SimpleLogger;
use std::{env, time::Duration};
use tokio::time::sleep;
#[tokio::main()]
async fn main() -> std::io::Result<()> {
    // load environment variables
    dotenv().ok();

    let explorer_server_port: String = env::var("EXPLORER_SERVER_PORT")
        .unwrap_or_else(|_| "8200".to_string());
    let explorer_server_host: String = env::var("EXPLORER_SERVER_HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string());

    // set configuration
    let log = SimpleLogger::new().with_level(LevelFilter::Info);
    let config = Config::new();

    if config.debug {
        log.with_level(LevelFilter::Debug).init().unwrap();
    } else {
        log.init().unwrap();
    }

    // set rpc
    let rpc = Rpc::new(&config).await;

    let db = Database::new(
        config.db_host.clone(),
        config.db_username.clone(),
        config.db_password.clone(),
        config.db_name.clone(),
        config.chain.clone(),
    )
    .await;

    // get new blocks
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

    // listend port for frontend
    let t = HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .configure(configure)
    })
    .bind(format!("{explorer_server_host}:{explorer_server_port}"))? // Bind server to localhost:8080
    .run();
    tokio::spawn(t);

    // sync chain
    loop {
        if !config.new_blocks_only {
            sync_chain(&rpc, &db, &config).await;
        }
        sleep(Duration::from_secs(30)).await;
    }
}
