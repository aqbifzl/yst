use std::sync::{Arc, Mutex};

use axum::Json;
use shared::storage::StorageJson;

use crate::watcher::storage::Storage;

pub async fn today(screen_time_watcher: Arc<Mutex<Storage>>) -> Json<StorageJson> {
    let data = screen_time_watcher.lock().unwrap().data.clone();

    axum::Json(data)
}
