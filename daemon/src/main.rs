use std::{
    process::exit,
    sync::{Arc, Mutex},
};

use daemon::{
    active_window::ActiveWinProperties,
    storage::Storage,
    utils::logger::{init_logger, log},
    watcher::watcher_main_loop,
    wayland_watcher::handle_wayland,
    x11_watcher::handle_x11,
};

fn main() {
    if let Err(err) = init_logger() {
        eprintln!("Failed to init logger: {}", err);
        exit(1);
    }
    log("Starting yst");

    let active_win = Arc::new(Mutex::new(ActiveWinProperties::new()));
    let is_afk = Arc::new(Mutex::new(false));
    let storage = Arc::new(Mutex::new(Storage::new()));

    #[cfg(feature = "x11")]
    handle_x11(active_win.clone(), is_afk.clone());

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

    watcher_main_loop(active_win, storage, is_afk);
}
