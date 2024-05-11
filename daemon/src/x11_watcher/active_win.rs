use crate::utils::process::get_name_by_pid;

use super::x11helper::X11Helper;

pub fn get_active_window_x11(x11_helper: &X11Helper) -> Option<(String, String)> {
    let active_window = x11_helper.get_root_window_id(x11_helper.atoms._net_active_window);

    if let Some(active_window) = active_window {
        let wm_name = x11_helper.get_window_wm_name(active_window); // alternative to net_name
        let net_name = x11_helper.get_window_net_name(active_window); // it takes priority
        let win_pid = x11_helper.get_number_property(active_window, x11_helper.atoms._net_wm_pid); //alternative to
                                                                                                   //program name
        let win_cmd_name = x11_helper.get_window_class_name(active_window); // program name

        let win_name = if net_name.is_some() {
            net_name
        } else {
            wm_name
        };

        let win_name = match win_name {
            Some(name) => name,
            None => return None,
        };

        if let Some(win_pid) = win_pid {
            if let Some(name) = get_name_by_pid(win_pid) {
                return Some((win_name, name));
            }
        }

        let win_cmd_name = match win_cmd_name {
            Some(name) => name,
            None => return None,
        };

        return Some((win_name, win_cmd_name));
    }

    None
}
