use std::env;
use std::sync::Arc;

mod connector;
mod db;
mod dto;
mod general_handler;
mod handlers;
mod helpers;
mod polling;
mod types;

use db::RedisRepo;
use polling::make_polling;

use crate::connector::TgConnector;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let db_con_str = env::var("REDIS_URL").expect("REDIS_URL env variable not provided");
    let token = env::var("TG_TOKEN").expect("TG_TOKEN env variable not provided");
    let api_url = env::var("API_URL").expect("API_URL env variable not provided");
    let polling_timeout: i64 = env::var("LP_TIMEOUT")
        .expect("LP_TIMEOUT env variable not provided")
        .parse()
        .expect("LP_TIMEOUT must be a number");

    let repo = RedisRepo::new(&db_con_str);

    let tg_con = TgConnector {
        client: reqwest::Client::new(),
        token,
        api_url,
        repo,
        polling_timeout,
    };

    make_polling(Arc::new(tg_con))
        .await
        .unwrap_or_else(|e| eprintln!("{}", e));

    Ok(())
}
