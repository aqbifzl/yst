use std::{
    error,
    sync::{Arc, Mutex},
};

use axum::{
    http::{HeaderValue, Method},
    routing, Router,
};
use shared::config::{API_HOST, CLIENT_URI};

use crate::{
    storage::Storage,
    utils::logger::{log_msg, LogLevel},
};
use tower_http::cors::CorsLayer;

use self::{day::day, ping::ping, today::today};

mod day;
mod ping;
mod today;

#[tokio::main]
pub async fn run_api(
    screen_time_watcher: Arc<Mutex<Storage>>,
) -> Result<(), Box<dyn error::Error>> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(CLIENT_URI.parse::<HeaderValue>().unwrap());

    let today_screen_watcher = screen_time_watcher.clone();
    let app = Router::new()
        .route("/ping", routing::get(ping))
        .route("/today", routing::get(|| today(today_screen_watcher)))
        .route("/day", routing::get(day))
        .layer(cors.clone());

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
