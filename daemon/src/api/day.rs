use std::{
    fs::read_to_string,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::from_str;
use shared::{storage::StorageJson, storage_utils::get_filepath};

use crate::utils::SECONDS_IN_DAY;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Params {
    timestamp: Option<i64>,
}

pub enum AppError {
    NotFound,
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NotFound => {
                (StatusCode::NOT_FOUND, "Not found activity from this day").into_response()
            }
            Self::InternalError(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
        }
    }
}

fn timestamp_to_json(timestamp: i64) -> Result<Json<StorageJson>, AppError> {
    let offset = (timestamp
        - SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AppError::InternalError("Internal error occurred".to_string()))?
            .as_secs() as i64)
        / SECONDS_IN_DAY as i64;

    let filepath = get_filepath(offset as i32, false).map_err(|_| AppError::NotFound)?;
    let str = read_to_string(filepath).map_err(|_| AppError::NotFound)?;
    let json: StorageJson = from_str(&str)
        .map_err(|_| AppError::InternalError("Internal error occurred".to_string()))?;

    Ok(Json(json))
}

pub async fn day(Query(params): Query<Params>) -> Result<Json<StorageJson>, AppError> {
    if let Some(timestamp) = params.timestamp {
        let json = timestamp_to_json(timestamp);
        return json;
    }

    Ok(axum::Json(StorageJson::new()))
}
