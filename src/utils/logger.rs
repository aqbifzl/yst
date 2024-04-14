use std::{
    fs::{self, metadata, OpenOptions},
    io::Write,
};

use crate::config::{LOG_PATH, LOG_SEPARATOR};

use super::{fs::escape_home_dir, get_current_date_and_time};

pub enum LogLevel {
    Info,
    Error,
    Debug,
}

pub fn log_msg(msg: &str, level: LogLevel) {
    let msg = format!(
        "{}{}{}",
        match level {
            LogLevel::Info => "INFO",
            LogLevel::Error => "ERROR",
            LogLevel::Debug => "DEBUG",
        },
        LOG_SEPARATOR,
        msg,
    );

    let escpaed_log_path = escape_home_dir(LOG_PATH).unwrap();
    let msg = get_current_date_and_time() + LOG_SEPARATOR + &msg + "\n";

    let mut file = OpenOptions::new()
        .append(true)
        .open(escpaed_log_path)
        .unwrap();

    file.write_all(msg.as_bytes()).unwrap();
}

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    let escpaed_log_path = escape_home_dir(LOG_PATH)?;
    let md = metadata(&escpaed_log_path);
    if md.is_err() {
        fs::File::create(escpaed_log_path)?;
    }

    Ok(())
}

pub fn log(msg: &str) {
    log_msg(msg, LogLevel::Info);
}
