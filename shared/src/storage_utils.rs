use std::{
    error::Error,
    fs::{create_dir_all, metadata, write},
    path::PathBuf,
};

use crate::{
    config::STORAGE_PATH,
    storage::StorageJson,
    utils::{escape_home_dir, get_specific_date},
};

pub fn get_filename(offset: i32) -> PathBuf {
    PathBuf::from(get_specific_date(offset) + ".json")
}

pub fn get_current_filepath(create: bool) -> Result<PathBuf, Box<dyn Error>> {
    get_filepath(0, create)
}

pub fn get_filepath(offset: i32, create: bool) -> Result<PathBuf, Box<dyn Error>> {
    let mut filepath = get_storage_dir()?;
    filepath.push(get_filename(offset));

    let md = metadata(&filepath);
    if md.is_err() {
        if !create {
            return Err("File not found".into());
        }
        write(&filepath, serde_json::to_string(&StorageJson::new())?)?;
    }

    Ok(filepath)
}

pub fn get_storage_dir() -> Result<PathBuf, Box<dyn Error>> {
    let escaped_home = escape_home_dir(STORAGE_PATH)?;
    let md = metadata(&escaped_home);
    if md.is_err() {
        create_dir_all(&escaped_home)?;
    }
    let md = metadata(&escaped_home)?;
    if md.permissions().readonly() {
        return Err("No permissions to write to storage path".into());
    }

    Ok(escaped_home)
}
