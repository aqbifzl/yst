use std::{
    process::exit,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use shared::config::{ACTIVITY_SAMPLING_RATE_MS, SAVE_INTERVAL_MS};

use crate::{
    active_window::ActiveWinProperties,
    storage::Storage,
    utils::logger::{log_msg, LogLevel},
};

pub fn watcher_main_loop(
    active_win: Arc<Mutex<ActiveWinProperties>>,
    storage: Arc<Mutex<Storage>>,
    is_afk: Arc<Mutex<bool>>,
) {
    storage
        .lock()
        .unwrap()
        .load_from_file()
        .unwrap_or_else(|_| {
            log_msg("Couldn't load data from file", LogLevel::Error);
            exit(1);
        });

    let time_to_wait = Duration::from_millis(ACTIVITY_SAMPLING_RATE_MS.into());
    let mut total_passed = Duration::default();

    loop {
        let is_afk = *is_afk.lock().unwrap();
        if is_afk {
            continue;
        }

        let props = *active_win.lock().unwrap();

        sleep(time_to_wait);
        total_passed += time_to_wait;

        let time_to_wait_ms = time_to_wait.as_millis();

        storage
            .lock()
            .unwrap()
            .data
            .titles
            .entry(props.name)
            .and_modify(|e| *e += time_to_wait_ms)
            .or_insert(time_to_wait_ms);
        storage
            .lock()
            .unwrap()
            .data
            .applications
            .entry(props.cmd)
            .and_modify(|e| *e += time_to_wait_ms)
            .or_insert(time_to_wait_ms);

        if total_passed.as_millis() % SAVE_INTERVAL_MS == 0 {
            storage.lock().unwrap().save_to_file().unwrap();
        }
    }
}
