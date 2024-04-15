use std::{
    process::exit,
    rc::Rc,
    sync::{Arc, Mutex},
    thread::spawn,
    time::Duration,
};

use daemon::{
    api::run_api,
    utils::logger::{init_logger, log, log_msg, LogLevel},
    watcher::{
        active_win::ActiveWinTracker, idle_watcher::IdleWatcher, storage::Storage,
        x11helper::X11Helper, ScreenTimeWatcher,
    },
};
use x11rb::{connect, connection::Connection};

fn main() {
    if let Err(err) = init_logger() {
        eprintln!("Failed to init logger: {}", err);
        exit(1);
    }
    log("Starting yst");

    let (connection, screen_num) = connect(None).unwrap_or_else(|_| {
        log_msg("Couldn't connect to X server", LogLevel::Error);
        exit(1);
    });
    log_msg("Connected to X from main thread", LogLevel::Debug);

    let screen = &connection.setup().roots[screen_num];
    let root = screen.root;
    let connection = Arc::new(Mutex::new(connection));
    let storage = Arc::new(Mutex::new(Storage::new()));
    log_msg("Successfully initialized storage", LogLevel::Debug);

    let x11_helper = match X11Helper::new(connection.clone()) {
        Ok(x11_helper) => x11_helper,
        Err(err) => {
            log_msg(
                &format!("Couldn't create new x11 helper: {}", err),
                LogLevel::Error,
            );
            exit(1);
        }
    };
    log_msg("Successfully initialized x11 helper", LogLevel::Debug);

    let active_win = ActiveWinTracker::new(&x11_helper, root);
    let idle_time = Arc::new(Mutex::new(Duration::default()));

    let watcher = IdleWatcher::new(idle_time.clone()).unwrap_or_else(|err| {
        log_msg(
            &format!("Couldn't init idle watcher: {}", &err.to_string()),
            LogLevel::Error,
        );
        exit(1);
    });
    log_msg("Successfully initialized idle watcher", LogLevel::Debug);

    spawn(move || watcher.run());

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

    let st_watcher =
        ScreenTimeWatcher::new(Rc::new(active_win), idle_time.clone(), storage.clone());

    let mut st_watcher = match st_watcher {
        Ok(watcher) => watcher,
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    };

    st_watcher.run();
}
