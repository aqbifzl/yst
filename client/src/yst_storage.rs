use std::{error::Error, fs::read_to_string};

use shared::{storage::StorageJson, storage_utils::get_filepath};

pub fn get_content(offset: i32) -> Result<StorageJson, Box<dyn Error>> {
    let filepath = get_filepath(offset, false)?;
    let str = read_to_string(filepath)?;
    let json: StorageJson = serde_json::from_str(&str)?;

    Ok(json)
}
