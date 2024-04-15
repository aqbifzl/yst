use std::{
    error::Error,
    process::exit,
    rc::Rc,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::{
    config::{ACTIVITY_SAMPLING_RATE_MS, MAX_IDLE_SECONDS, SAVE_INTERVAL_MS},
    utils::logger::{log_msg, LogLevel},
};

use self::{
    active_win::{ActiveWin, ActiveWinTracker},
    storage::Storage,
};

pub mod active_win;
pub mod idle_watcher;
pub mod storage;
pub mod x11helper;

pub struct ScreenTimeWatcher<'a> {
    storage: Arc<Mutex<Storage>>,
    win_tracker: Rc<ActiveWinTracker<'a>>,
    idle_time: Arc<Mutex<Duration>>,
}

impl<'a> ScreenTimeWatcher<'a> {
    pub fn new(
        win_tracker: Rc<ActiveWinTracker<'a>>,
        idle_time: Arc<Mutex<Duration>>,
        storage: Arc<Mutex<Storage>>,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            win_tracker,
            idle_time: idle_time.clone(),
            storage,
        })
    }
    pub fn run(&mut self) {
        self.storage
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
            let idle_time = *self.idle_time.lock().unwrap();

            if idle_time > Duration::from_secs(MAX_IDLE_SECONDS.into()) {
                continue;
            }

            let props = match self.win_tracker.get() {
                ActiveWin::Properties(props) => props,
                ActiveWin::None => {
                    sleep(time_to_wait);
                    continue;
                }
            };

            sleep(time_to_wait);
            total_passed += time_to_wait;

            let time_to_wait_ms = time_to_wait.as_millis();
            self.storage
                .lock()
                .unwrap()
                .data
                .titles
                .entry(props.name)
                .and_modify(|e| *e += time_to_wait_ms)
                .or_insert(time_to_wait_ms);
            self.storage
                .lock()
                .unwrap()
                .data
                .applications
                .entry(props.cmd)
                .and_modify(|e| *e += time_to_wait_ms)
                .or_insert(time_to_wait_ms);

            if total_passed.as_millis() % SAVE_INTERVAL_MS == 0 {
                self.storage.lock().unwrap().save_to_file().unwrap();
            }
        }
    }
}
