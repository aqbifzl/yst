use std::{
    error::Error,
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

use shared::config::MAX_IDLE_SECONDS;
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

use crate::utils::logger::{log_msg, LogLevel};

fn select_xi_events(conn: &RustConnection, win: Window) -> Result<(), Box<dyn Error>> {
    xinput::xi_query_version(conn, 2, 3)?;

    conn.xinput_xi_select_events(
        win,
        &[xinput::EventMask {
            deviceid: xinput::Device::ALL_MASTER.into(),
            mask: vec![
                (XIEventMask::RAW_KEY_PRESS
                    | XIEventMask::RAW_KEY_RELEASE
                    | XIEventMask::RAW_BUTTON_PRESS
                    | XIEventMask::RAW_BUTTON_RELEASE
                    | XIEventMask::RAW_MOTION),
            ],
        }],
    )?
    .check()?;

    Ok(())
}

pub fn idle_loop_x11(
    is_afk: Arc<Mutex<bool>>,
    connection: Arc<Mutex<RustConnection>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (connection, screen_num) = connect(None)?;

    let screen = &connection.setup().roots[screen_num];
    select_xi_events(&connection, screen.root)?;

    log_msg("Running loop monitoring inactivity", LogLevel::Debug);
    let afk_duration = Arc::new(Mutex::new(Duration::default()));

    let zero_counter = afk_duration.clone();
    let is_afk_for_zero = is_afk.clone();
    let zero_conn = connection.clone();
    let zero_th = spawn(move || loop {
        let ev = zero_conn.lock().unwrap().wait_for_event();
        let ev = match ev {
            Ok(ev) => ev,
            Err(ev_err) => {
                log_msg(&format!("{}", ev_err), LogLevel::Error);
                continue;
            }
        };
        match ev {
            XinputRawKeyPress(_)
            | XinputRawKeyRelease(_)
            | XinputRawButtonPress(_)
            | XinputRawButtonRelease(_)
            | XinputRawMotion(_) => {
                *is_afk_for_zero.lock().unwrap() = false;
                *zero_counter.lock().unwrap() = Duration::default();
            }
            _ => (),
        }
    });

    let counter = afk_duration.clone();
    let is_afk_for_counter = is_afk.clone();
    let count_th = spawn(move || loop {
        sleep(Duration::from_secs(1));
        *counter.lock().unwrap() += Duration::from_secs(1);

        if *counter.lock().unwrap() > MAX_IDLE_SECONDS {
            *is_afk_for_counter.lock() = true;
        }
    });

    zero_th.join().unwrap();
    count_th.join().unwrap();

    Ok(())
}
