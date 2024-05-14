use std::{
    process::exit,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use shared::config::{ACTIVITY_SAMPLING_RATE_MS, SAVE_INTERVAL_MS};

use crate::{
    active_window::ActiveWindow,
    storage::Storage,
    utils::logger::{log_msg, LogLevel},
    wayland_watcher::toplevel_handler::get_current_name_and_cmd,
};

pub fn watcher_main_loop(
    active_win: &mut ActiveWindow,
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
        if *is_afk.lock().unwrap() {
            continue;
        }

        #[cfg(feature = "wayland")]
        {
            let (new_name, new_cmd) = get_current_name_and_cmd();
            if new_cmd.is_none() || new_name.is_none() {
                continue;
            }
            active_win.name = new_name;
            active_win.cmd = new_cmd;
        }

        #[cfg(feature = "x11")]
        active_win.get();
        let default_value = || "unknown".to_string();

        let (name, cmd) = (active_win.name.as_ref(), active_win.cmd.as_ref());
        let (name, cmd) = (
            name.map_or_else(default_value, |c| c.to_string()),
            cmd.map_or_else(default_value, |c| c.to_string()),
        );

        sleep(time_to_wait);
        total_passed += time_to_wait;

        let time_to_wait_ms = time_to_wait.as_millis();

        storage
            .lock()
            .unwrap()
            .data
            .titles
            .entry(name)
            .and_modify(|e| *e += time_to_wait_ms)
            .or_insert(time_to_wait_ms);

        storage
            .lock()
            .unwrap()
            .data
            .applications
            .entry(cmd)
            .and_modify(|e| *e += time_to_wait_ms)
            .or_insert(time_to_wait_ms);

        if total_passed.as_millis() % SAVE_INTERVAL_MS == 0 {
            storage.lock().unwrap().save_to_file().unwrap();
        }
    }
}
