use std::sync::{Arc, Mutex};

use axum::{routing, Router};

use crate::{config::API_HOST, watcher::storage::Storage};

use self::{ping::ping, today::today};

mod ping;
mod today;

#[tokio::main]
pub async fn run_api(screen_time_watcher: Arc<Mutex<Storage>>) {
    let app = Router::new()
        .route("/ping", routing::get(ping))
        .route("/today", routing::get(|| today(screen_time_watcher)));

    let listener = tokio::net::TcpListener::bind(API_HOST).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
