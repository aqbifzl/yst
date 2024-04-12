use chrono::{self};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, create_dir_all, metadata, read_to_string},
    path::PathBuf,
};

use crate::{config::STORAGE_PATH, utils::fs::escape_home_dir};

// use crate::{config::STORAGE_PATH, fs_utils::escape_home_dir};

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

impl StorageJson {
    fn new() -> Self {
        Self {
            applications: HashMap::new(),
            titles: HashMap::new(),
        }
    }
}

pub struct Storage {
    pub data: StorageJson,
    current_path: PathBuf,
    storage_dir: PathBuf,
}

fn get_current_date() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

pub fn get_todays_filename() -> PathBuf {
    PathBuf::from(get_current_date() + ".json")
}

impl Storage {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut storage = Self {
            data: StorageJson::new(),
            current_path: PathBuf::new(), // current_path:
            storage_dir: Self::get_storage_dir()?,
        };
        storage.init_storage()?;
        Ok(storage)
    }
    pub fn print(&self) {
        println!("applications:");
        for (key, value) in self.data.applications.iter() {
            println!("{} -> {}ms", key, value);
        }
        println!("names:");
        for (key, value) in self.data.titles.iter() {
            println!("{} -> {}ms", key, value);
        }
    }
    pub fn load_from_file(&mut self) -> Result<(), Box<dyn Error>> {
        let filename_path = self.get_current_filename()?;
        let str = read_to_string(&filename_path)?;

        let json: StorageJson = serde_json::from_str(&str)?;

        self.data = json;
        self.current_path = filename_path;

        Ok(())
    }
    pub fn save_to_file(&mut self) -> Result<(), Box<dyn Error>> {
        let filename_path = self.get_current_filename()?;
        if filename_path != self.current_path {
            // day changed
            self.load_from_file()?;
            return Ok(());
        }

        let str = serde_json::to_string(&self.data)?;

        fs::write(&filename_path, str)?;

        Ok(())
    }
    pub fn get_current_filename(&self) -> Result<PathBuf, Box<dyn Error>> {
        let mut storage_dir = self.storage_dir.clone();
        storage_dir.push(get_todays_filename());
        Ok(storage_dir)
    }
    fn get_storage_dir() -> Result<PathBuf, Box<dyn Error>> {
        let escaped_home = escape_home_dir(STORAGE_PATH)?;
        let md = fs::metadata(&escaped_home);
        if md.is_err() {
            create_dir_all(&escaped_home)?;
        }
        let md = metadata(&escaped_home)?;
        if md.permissions().readonly() {
            return Err("No permissions to write to storage path".into());
        }

        Ok(escaped_home)
    }
    pub fn init_storage(&mut self) -> Result<(), Box<dyn Error>> {
        let filename_path = self.get_current_filename()?;

        if metadata(&filename_path).is_ok() {
            return Ok(());
        }

        fs::write(&filename_path, serde_json::to_string(&StorageJson::new())?)?;
        self.current_path = filename_path;

        Ok(())
    }
}
