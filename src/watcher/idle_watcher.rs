use std::{
    error::{self, Error},
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

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

pub struct IdleWatcher {
    pub idle_time: Arc<Mutex<Duration>>,
    connection: Arc<Mutex<RustConnection>>,
}

impl IdleWatcher {
    pub fn new(duration: Arc<Mutex<Duration>>) -> Result<Self, Box<dyn error::Error>> {
        let (connection, screen_num) = connect(None)?;

        let screen = &connection.setup().roots[screen_num];
        Self::select_xi_events(&connection, screen.root)?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            idle_time: duration.clone(),
        })
    }
    pub fn run(&self) {
        log_msg("Running loop monitoring inactivity", LogLevel::Debug);
        let zero_it = self.idle_time.clone();
        let zero_conn = self.connection.clone();
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
                    *zero_it.lock().unwrap() = Duration::default();
                }
                _ => (),
            }
        });

        let count_it = self.idle_time.clone();
        let count_th = spawn(move || loop {
            sleep(Duration::from_secs(1));
            *count_it.lock().unwrap() += Duration::from_secs(1);
        });

        zero_th.join().unwrap();
        count_th.join().unwrap();
    }
    pub fn select_xi_events(conn: &RustConnection, win: Window) -> Result<(), Box<dyn Error>> {
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
}
