use actix_cors::Cors;
use actix_web::{
    http, middleware,
    web::{self, Data},
    App, HttpServer,
};
use args::*;
use clap::Parser;
use state::*;
use std::sync::Arc;
use subxt::{client::OnlineClient, PolkadotConfig};
use util::url_to_string;

mod account;
mod args;
mod asset;
mod bag;
mod bundle;
mod challenge;
mod config;
mod contract;
mod fula;
mod market;
mod pool;
mod state;
mod subscription;
mod util;
mod validator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let args = Args::parse();

    let api = OnlineClient::<PolkadotConfig>::from_url(url_to_string(args.node_server))
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let state = AppState { api: Arc::new(api) };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(Data::new(state.clone()))
            .service(web::resource("/ws").route(web::get().to(subscription::ws)))
            .route("health", web::post().to(util::health_check))
            .route("account/seeded", web::post().to(account::seeded))
            .route("account/exists", web::post().to(account::exists))
            .route("account/create", web::post().to(account::create))
            .route("account/fund", web::post().to(account::fund))
            .route("account/balance", web::post().to(account::balance))
            .route("asset/create_class", web::post().to(asset::create_class))
            .route("asset/class_info", web::post().to(asset::class_info))
            .route("asset/create", web::post().to(asset::create))
            .route("asset/info", web::post().to(asset::info))
            .route(
                "asset/update_metadata",
                web::post().to(asset::update_metadata),
            )
            .route("asset/mint", web::post().to(asset::mint))
            .route("asset/burn", web::post().to(asset::burn))
            .route("asset/balance", web::post().to(asset::balance))
            .route("asset/balances", web::post().to(asset::balances))
            .route("asset/transfer_from", web::post().to(asset::transfer_from))
            .route("bag/register", web::post().to(bag::register))
            .route("bag/create", web::post().to(bag::create))
            .route("bag/sweep", web::post().to(bag::sweep))
            .route("bag/deposit", web::post().to(bag::deposit))
            .route("bundle/register", web::post().to(bundle::register_bundle))
            .route("bundle/mint", web::post().to(bundle::mint_bundle))
            .route("bundle/burn", web::post().to(bundle::burn_bundle))
            .route("bundle", web::post().to(bundle::get_bundles_id))
            .route("bundle/data", web::post().to(bundle::get_bundles_data))
            .route(
                "validator/add_validator",
                web::post().to(validator::add_validator),
            )
            .route(
                "validator/remove_validator",
                web::post().to(validator::remove_validator),
            )
            .route(
                "market/create_market",
                web::post().to(market::create_market),
            )
            .route(
                "market/create_market_rate",
                web::post().to(market::create_market_rate),
            )
            .route(
                "market/deposit_assets",
                web::post().to(market::deposit_assets),
            )
            .route(
                "market/exchange_assets",
                web::post().to(market::exchange_assets),
            )
            .route(
                "fula/manifest/update",
                web::post().to(fula::update_manifest),
            )
            .route("fula/manifest", web::post().to(fula::get_all_manifests))
            .route(
                "fula/manifest/alter",
                web::post().to(fula::get_all_manifests_alter),
            )
            .route(
                "fula/manifest/remove",
                web::post().to(fula::remove_manifest),
            )
            .route(
                "fula/manifest/batch_remove",
                web::post().to(fula::batch_remove_manifest),
            )
            .route(
                "fula/manifest/remove_stored_manifest",
                web::post().to(fula::remove_stored_manifest),
            )
            .route(
                "fula/manifest/batch_remove_stored_manifest",
                web::post().to(fula::batch_remove_stored_manifest),
            )
            .route(
                "fula/manifest/upload",
                web::post().to(fula::upload_manifest),
            )
            .route(
                "fula/manifest/batch_upload",
                web::post().to(fula::batch_upload_manifest),
            )
            .route(
                "fula/manifest/available",
                web::post().to(fula::get_available_manifests),
            )
            .route(
                "fula/manifest/available/alter",
                web::post().to(fula::get_all_available_manifests_alter),
            )
            .route(
                "fula/manifest/storage",
                web::post().to(fula::storage_manifest),
            )
            .route(
                "fula/manifest/batch_storage",
                web::post().to(fula::batch_storage_manifest),
            )
            .route(
                "fula/manifest/storer_data",
                web::post().to(fula::get_all_manifests_storer_data),
            )
            .route(
                "fula/manifest/storer_data/alter",
                web::post().to(fula::get_all_manifests_storer_data_alter),
            )
            .route(
                "fula/manifest/verify",
                web::post().to(fula::verify_manifest),
            )
            .route("fula/pool/create", web::post().to(pool::create_pool))
            .route("fula/pool/leave", web::post().to(pool::leave_pool))
            .route("fula/pool/join", web::post().to(pool::join_pool))
            .route(
                "fula/pool/cancel_join",
                web::post().to(pool::cancel_join_pool),
            )
            .route("fula/pool/vote", web::post().to(pool::vote))
            .route("fula/pool", web::post().to(pool::get_all_pools))
            .route(
                "fula/pool/poolrequests",
                web::post().to(pool::get_all_pool_requests),
            )
            .route("fula/pool/users", web::post().to(pool::get_all_pool_users))
            .route(
                "fula/contract/mint",
                web::post().to(contract::contract_mint_to),
            )
            .route(
                "fula/contract/supply",
                web::post().to(contract::contract_total_supply),
            )
            .route(
                "fula/contract/allowance",
                web::post().to(contract::contract_allowance),
            )
            .route(
                "fula/contract/increase_allowance",
                web::post().to(contract::contract_increase_allowance),
            )
            .route(
                "fula/contract/decrease_allowance",
                web::post().to(contract::contract_decrease_allowance),
            )
            .route(
                "fula/contract/burn",
                web::post().to(contract::contract_burn_from),
            )
            .route(
                "fula/contract/transfer",
                web::post().to(contract::contract_transfer),
            )
            .route(
                "fula/challenge/generate",
                web::post().to(challenge::generate_challenge),
            )
            .route(
                "fula/challenge/verify",
                web::post().to(challenge::verify_challenge),
            )
            .route(
                "fula/mint_labor_tokens",
                web::post().to(challenge::mint_labor_tokens),
            )
            .route(
                "fula/challenge/pending",
                web::post().to(challenge::verify_pending_challenge),
            )
            .route(
                "fula/file/verify",
                web::post().to(challenge::verify_file_size),
            )
            .route(
                "fula/file/provide",
                web::post().to(challenge::provide_file_size),
            )
            .route("fula/challenge", web::post().to(challenge::get_challenges))
            .route("fula/claims", web::post().to(challenge::get_claims))
            .route(
                "fula/goerli/convert_tokens",
                web::post().to(contract::goerli_convert_to_fula),
            )
            .route(
                "fula/mumbai/convert_tokens",
                web::post().to(contract::mumbai_convert_to_fula),
            )
    })
    .bind((args.listen.host_str().unwrap(), args.listen.port().unwrap()))?
    .run()
    .await
}
