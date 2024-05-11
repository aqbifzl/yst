use crate::x11_watcher::x11helper::X11Helper;

pub struct ActiveWindow {
    #[cfg(feature = "x11")]
    pub x11_helper: X11Helper,
    #[cfg(feature = "x11")]
    pub root: u32,

    pub name: Option<String>,
    pub cmd: Option<String>,
}

impl ActiveWindow {
    #[cfg(feature = "x11")]
    pub fn new(x11_helper: X11Helper, root: u32) -> Self {
        Self {
            name: None,
            cmd: None,
            x11_helper,
            root,
        }
    }
    #[cfg(feature = "x11")]
    pub fn get(&mut self) {
        use crate::x11_watcher::active_win::get_active_window_x11;
        match get_active_window_x11(&self.x11_helper, self.root) {
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
