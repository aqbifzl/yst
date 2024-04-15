use std::{
    error,
    sync::{Arc, Mutex},
};

use axum::{routing, Router};

use crate::{
    config::API_HOST,
    utils::logger::{log_msg, LogLevel},
    watcher::storage::Storage,
};

use self::{ping::ping, today::today};

mod ping;
mod today;

#[tokio::main]
pub async fn run_api(
    screen_time_watcher: Arc<Mutex<Storage>>,
) -> Result<(), Box<dyn error::Error>> {
    let app = Router::new()
        .route("/ping", routing::get(ping))
        .route("/today", routing::get(|| today(screen_time_watcher)));

    let listener = match tokio::net::TcpListener::bind(API_HOST).await {
        Ok(listener) => listener,
        Err(err) => {
            return Err(err.into());
        }
    };

    log_msg(
        &format!("Running api server on {}", API_HOST),
        LogLevel::Debug,
    );

    if let Err(err) = axum::serve(listener, app).await.map_err(Box::new) {
        return Err(err.into());
    }

    Ok(())
}
