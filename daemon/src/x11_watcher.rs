use std::{
    process::exit,
    sync::{Arc, Mutex},
    thread::spawn,
};

// use x11rb::connect;

use x11rb::{
    connect,
    connection::Connection,
    protocol::{
        xinput::{self, ConnectionExt, XIEventMask},
        xproto::Window,
        Event::{
            XinputRawButtonPress, XinputRawButtonRelease, XinputRawKeyPress, XinputRawKeyRelease,
            XinputRawMotion,
        },
    },
    rust_connection::RustConnection,
};

use crate::{
    active_window::ActiveWindow,
    utils::logger::{log_msg, LogLevel},
};

use self::{idle_watcher::idle_loop_x11, x11helper::X11Helper};

pub mod active_win;
pub mod idle_watcher;
pub mod x11helper;

pub fn handle_x11(is_afk: Arc<Mutex<bool>>) -> ActiveWindow {
    let (connection, screen_num) = connect(None).unwrap_or_else(|_| {
        log_msg("Couldn't connect to X server", LogLevel::Error);
        exit(1);
    });
    log_msg("Connected to X from main thread", LogLevel::Debug);

    let screen = &connection.setup().roots[screen_num];
    let root = screen.root;
    let connection = Arc::new(Mutex::new(connection));

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

    let active_win = ActiveWindow::new(x11_helper, root);

    spawn(move || {
        idle_loop_x11(is_afk.clone(), connection.clone(), root).unwrap_or_else(|err| {
            log_msg(
                &format!("Couldn't init idle watcher: {}", &err.to_string()),
                LogLevel::Error,
            );
            exit(1);
        });
    });

    active_win
}
