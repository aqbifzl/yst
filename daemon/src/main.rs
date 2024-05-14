use std::{
    process::exit,
    sync::{Arc, Mutex},
    thread::spawn,
};

use daemon::{
    active_window::ActiveWindow,
    api::run_api,
    storage::Storage,
    utils::logger::{init_logger, log, log_msg, LogLevel},
    watcher::watcher_main_loop,
};

#[cfg(feature = "wayland")]
use daemon::wayland_watcher::handle_wayland;
#[cfg(feature = "x11")]
use daemon::{watcher::watcher_main_loop, x11_watcher::handle_x11};

fn main() {
    if let Err(err) = init_logger() {
        eprintln!("Failed to init logger: {}", err);
        exit(1);
    }
    log("Starting yst");

    let is_afk = Arc::new(Mutex::new(false));
    let storage = Arc::new(Mutex::new(Storage::new()));

    #[cfg(feature = "x11")]
    let mut active_win = handle_x11(is_afk.clone());

    #[cfg(feature = "wayland")]
    let mut active_win = ActiveWindow::default(); // dummy
    #[cfg(feature = "wayland")]
    handle_wayland();

    let api_storage_clone = storage.clone();
    #[cfg(feature = "api")]
    spawn(move || {
        if let Err(err) = run_api(api_storage_clone) {
            log_msg(
                &format!("Error running api server: {}", err),
                LogLevel::Error,
            );
        }
    });

    watcher_main_loop(&mut active_win, storage, is_afk);
}
