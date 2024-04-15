use shared::{storage::StorageJson, storage_utils::get_current_filepath};
use std::{
    error::Error,
    fs::{self, read_to_string},
    path::PathBuf,
};

pub struct Storage {
    pub data: StorageJson,
    last_filepath: PathBuf,
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: StorageJson::new(),
            last_filepath: PathBuf::new(),
        }
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
        let filepath = get_current_filepath()?;
        let str = read_to_string(&filepath)?;
        let json: StorageJson = serde_json::from_str(&str)?;

        self.data = json;
        self.last_filepath = filepath;

        Ok(())
    }
    pub fn save_to_file(&mut self) -> Result<(), Box<dyn Error>> {
        let filepath = get_current_filepath()?;
        if filepath != self.last_filepath {
            self.data.applications.clear();
            self.data.titles.clear();
            self.last_filepath = filepath;
            return Ok(());
        }
        let str = serde_json::to_string(&self.data)?;

        fs::write(filepath, str)?;

        Ok(())
    }
}
