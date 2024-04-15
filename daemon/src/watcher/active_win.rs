use x11rb::protocol::xproto::Window;

use crate::utils::process::get_name_by_pid;

use super::x11helper::X11Helper;

pub struct ActiveWinProperties {
    pub name: String,
    pub cmd: String,
}

impl ActiveWinProperties {
    pub fn new(name: &str, cmd: &str) -> Self {
        Self {
            name: name.to_string(),
            cmd: cmd.to_string(),
        }
    }
}

pub enum ActiveWin {
    None,
    Properties(ActiveWinProperties),
}

pub struct ActiveWinTracker<'a> {
    x11_helper: &'a X11Helper,
    root: Window,
}

impl<'a> ActiveWinTracker<'a> {
    pub fn new(x11_helper: &'a X11Helper, root: Window) -> Self {
        Self { x11_helper, root }
    }
    pub fn get(&self) -> ActiveWin {
        let active_window = self
            .x11_helper
            .get_window_id_from_atom(self.root, self.x11_helper.atoms._net_active_window);

        if let Some(active_window) = active_window {
            let wm_name = self.x11_helper.get_window_wm_name(active_window); // alternative to net_name
            let net_name = self.x11_helper.get_window_net_name(active_window); // it takes priority
            let win_pid = self
                .x11_helper
                .get_number_property(active_window, self.x11_helper.atoms._net_wm_pid); //alternative to
                                                                                        //program name
            let win_cmd_name = self.x11_helper.get_window_class_name(active_window); // program name

            let win_name = if net_name.is_some() {
                net_name
            } else {
                wm_name
            };

            let win_name = match win_name {
                Some(name) => name,
                None => return ActiveWin::None,
            };

            if let Some(win_pid) = win_pid {
                if let Some(name) = get_name_by_pid(win_pid) {
                    return ActiveWin::Properties(ActiveWinProperties::new(&win_name, &name));
                }
            }

            let win_cmd_name = match win_cmd_name {
                Some(name) => name,
                None => return ActiveWin::None,
            };

            return ActiveWin::Properties(ActiveWinProperties::new(&win_name, &win_cmd_name));
        }

        ActiveWin::None
    }
}
