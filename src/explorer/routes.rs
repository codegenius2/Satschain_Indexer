use crate::explorer::handlers::*;
use actix_web::web;
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(index)) // GET request to "/"
            .route("/status", web::get().to(status)) // GET request to "/status"
            .route("/api/v2/blocks", web::get().to(handle_get_blocks))
            .route(
                "/api/v2/blocks/{id}",
                web::get().to(handle_get_block_by_id),
            )
            .route(
                "/api/v2/transactions",
                web::get().to(handle_get_transactions),
            )
            .route(
                "/api/v2/transactions/{id}",
                web::get().to(handle_get_transaction_by_id),
            )
            .route("/api/v2/stats", web::get().to(handle_get_stats))
            .route(
                "/api/v2/main-page/blocks",
                web::get().to(handle_main_page_blocks),
            )
            .route(
                "/api/v2/main-page/transactions",
                web::get().to(handle_main_page_transactions),
            )
            .route(
                "/api/v2/stats/charts/transactions",
                web::get().to(handle_get_stats_charts_transactions),
            )
            .route(
                "/api/eth_get_balance",
                web::get().to(handle_eth_get_balance),
            )
            .route("/api/balance", web::get().to(handle_balance))
            .route("/api/balancemulti", web::get().to(handle_balancemulti))
            .route(
                "/api/pendingtxlist",
                web::get().to(handle_pendingtxlist),
            )
            .route("/api/txlist", web::get().to(handle_txlist))
            .route(
                "/api/txlistinternal",
                web::get().to(handle_txlistinternal),
            )
            .route("/api/tokentx", web::get().to(handle_tokentx))
            .route("/api/tokenbalance", web::get().to(handle_tokenbalance))
            .route(
                "/api/getblockreward",
                web::get().to(handle_getblockreward),
            )
            .route(
                "/api/getblockcountdown",
                web::get().to(handle_getblockcountdown),
            )
            .route(
                "/api/getblocknobytime",
                web::get().to(handle_getblocknobytime),
            )
            .route(
                "/api/eth_block_number",
                web::get().to(handle_eth_block_number),
            )
            .route(
                "/api/listcontracts",
                web::get().to(handle_listcontracts),
            )
            .route("/api/getabi", web::get().to(handle_getabi))
            .route(
                "/api/getsourcecode",
                web::get().to(handle_getsourcecode),
            )
            .route(
                "/api/getcontractcreation",
                web::get().to(handle_getcontractcreation),
            )
            .route("/api/verify", web::get().to(handle_verify))
            .route(
                "/api/verify_via_sourcify",
                web::get().to(handle_verify_via_sourcify),
            )
            .route(
                "/api/verify_vyper_contract",
                web::get().to(handle_verify_vyper_contract),
            )
            .route(
                "/api/verifysourcecode",
                web::get().to(handle_verifysourcecode),
            )
            .route(
                "/api/checkverifystatus",
                web::get().to(handle_checkverifystatus),
            )
            .route(
                "/api/verifyproxycontract",
                web::get().to(handle_verifyproxycontract),
            )
            .route(
                "/api/checkproxyverification",
                web::get().to(handle_checkproxyverification),
            )
            .route("/api/get_logs", web::get().to(handle_get_logs))
            .route("/api/token_supply", web::get().to(handle_token_supply))
            .route("/api/get_token", web::get().to(handle_get_token))
            .route(
                "/api/get_token_holders",
                web::get().to(handle_get_token_holders),
            )
            .route(
                "/api/bridged_token_list",
                web::get().to(handle_bridged_token_list),
            )
            .route("/api/get_tx_info", web::get().to(handle_get_tx_info))
            .route(
                "/api/get_tx_receipt_status",
                web::get().to(handle_get_tx_receipt_status),
            )
            .route("/api/get_status", web::get().to(handle_get_status)),
    );
}
