use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct StorageEntry {
    identifier: String,
    duration: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StorageJson {
    pub applications: HashMap<String, u128>,
    pub titles: HashMap<String, u128>,
}

impl Default for StorageJson {
    fn default() -> Self {
        StorageJson::new()
    }
}

impl StorageJson {
    pub fn new() -> Self {
        Self {
            applications: HashMap::new(),
            titles: HashMap::new(),
        }
    }
}
