use std::{env, error::Error, path::PathBuf};

pub fn escape_home_dir(str: &str) -> Result<PathBuf, Box<dyn Error>> {
    match str.find('~') {
        Some(index) => {
            if index != 0 {
                return Ok(PathBuf::from(str));
            }

            let str: String = str.to_string();

            let homedir = env!("HOME");

            Ok(PathBuf::from(str.replacen('~', homedir, 1)))
        }
        None => Ok(PathBuf::from(str)),
    }
}
