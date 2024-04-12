use std::{
    process::exit,
    rc::Rc,
    sync::{Arc, Mutex},
    thread::spawn,
    time::Duration,
};

use x11rb::{connect, connection::Connection};

#[cfg(feature = "api")]
use yst::api::run_api;

use yst::watcher::{
    active_win::ActiveWinTracker, idle_watcher::IdleWatcher, storage::Storage,
    x11helper::X11Helper, ScreenTimeWatcher,
};

fn main() {
    let (connection, screen_num) = connect(None).unwrap();
    let screen = &connection.setup().roots[screen_num];
    let root = screen.root;
    let connection = Arc::new(Mutex::new(connection));
    let storage = match Storage::new() {
        Ok(storage) => Arc::new(Mutex::new(storage)),
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    };

    let x11_helper = X11Helper::new(connection.clone());
    let active_win = ActiveWinTracker::new(&x11_helper, root);
    let idle_time = Arc::new(Mutex::new(Duration::default()));

    let watcher = IdleWatcher::new(idle_time.clone());

    spawn(move || {
        watcher.run();
    });

    let api_storage_clone = storage.clone();
    #[cfg(feature = "api")]
    spawn(move || {
        run_api(api_storage_clone);
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
