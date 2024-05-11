use std::{
    process::exit,
    sync::{Arc, Mutex},
    thread::spawn,
};

use crate::{
    active_window::ActiveWindow,
    utils::logger::{log_msg, LogLevel},
};

use self::idle_watcher::idle_loop_x11;

pub mod active_win;
pub mod idle_watcher;
pub mod x11helper;

pub fn handle_x11(is_afk: Arc<Mutex<bool>>) -> ActiveWindow {
    let active_win = ActiveWindow::new();

    spawn(move || {
        idle_loop_x11(is_afk.clone()).unwrap_or_else(|err| {
            log_msg(
                &format!("Couldn't init idle watcher: {}", &err.to_string()),
                LogLevel::Error,
            );
            exit(1);
        });
    });

    active_win
}
