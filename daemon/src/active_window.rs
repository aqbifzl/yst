use crate::x11_watcher::x11helper::X11Helper;

pub struct ActiveWindow {
    #[cfg(feature = "x11")]
    pub x11_helper: X11Helper,

    pub name: Option<String>,
    pub cmd: Option<String>,
}

impl Default for ActiveWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl ActiveWindow {
    #[cfg(feature = "x11")]
    pub fn new() -> Self {
        use std::process::exit;

        use crate::utils::logger::{log_msg, LogLevel};

        let x11_helper = match X11Helper::new() {
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

        Self {
            name: None,
            cmd: None,
            x11_helper,
        }
    }
    #[cfg(feature = "x11")]
    pub fn get(&mut self) {
        use crate::x11_watcher::active_win::get_active_window_x11;
        match get_active_window_x11(&self.x11_helper) {
            Some((name, cmd)) => {
                self.name = Some(name.to_string());
                self.cmd = Some(cmd.to_string());
            }
            None => {
                self.name = None;
                self.cmd = None;
            }
        }
    }
}
