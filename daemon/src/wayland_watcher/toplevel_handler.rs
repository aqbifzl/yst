use std::sync::Arc;
use std::{collections::HashMap, sync::Mutex};

use crate::wl_client::toplevel::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1 as ToplevelHandle;
use crate::wl_client::toplevel::zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1 as ToplevelManager;

use crate::wl_client::toplevel::zwlr_foreign_toplevel_handle_v1::Event as TopLevelHandleEvent;

use lazy_static::lazy_static;
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};

use super::YstState;

#[derive(Default, Debug, Clone)]
pub struct Window {
    pub title: Option<String>,
    pub app_id: Option<String>,
    pub state: Option<Vec<u8>>,
}

lazy_static! {
    pub static ref WINDOWS_STATE: Arc<Mutex<HashMap<String, Window>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref CURRENT_WINDOW_ID: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

pub fn get_current_name_and_cmd() -> (Option<String>, Option<String>) {
    let current_window_id = match CURRENT_WINDOW_ID.lock().unwrap().clone() {
        Some(id) => id,
        None => return (None, None),
    };

    let current_win = WINDOWS_STATE
        .lock()
        .unwrap()
        .get(&current_window_id)
        .unwrap()
        .clone();

    match current_win {
        Window {
            title: Some(title),
            app_id: Some(app_id),
            state: Some(state),
        } => {
            let is_focused = state.iter().any(|&e| e == 2);
            if is_focused {
                (
                    Some(title.clone().to_string()),
                    Some(app_id.clone().to_string()),
                )
            } else {
                (None, None)
            }
        }
        _ => return (None, None),
    }
}

impl Dispatch<ToplevelHandle, ()> for YstState {
    fn event(
        _: &mut Self,
        proxy: &ToplevelHandle,
        event: <ToplevelHandle as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let id = proxy.id().to_string();

        match event {
            TopLevelHandleEvent::Title { title } => {
                WINDOWS_STATE.lock().unwrap().entry(id).or_default().title = Some(title);
            }
            TopLevelHandleEvent::AppId { app_id } => {
                WINDOWS_STATE.lock().unwrap().entry(id).or_default().app_id = Some(app_id);
            }
            TopLevelHandleEvent::State { state } => {
                WINDOWS_STATE.lock().unwrap().entry(id).or_default().state = Some(state);
            }
            TopLevelHandleEvent::Done => {
                let window = WINDOWS_STATE.lock().unwrap();
                let window = window.get(&id);
                match window {
                    Some(Window {
                        title: Some(_),
                        app_id: Some(_),
                        state: Some(state),
                    }) => {
                        let is_focused = state.iter().any(|&e| e == 2);
                        if is_focused {
                            *CURRENT_WINDOW_ID.lock().unwrap() = Some(id);
                        } else {
                            *CURRENT_WINDOW_ID.lock().unwrap() = None;
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

impl Dispatch<ToplevelManager, ()> for YstState {
    wayland_client::event_created_child!(YstState, ToplevelManager, [
        _ => (ToplevelHandle, ())
    ]);
    fn event(
        _: &mut Self,
        _: &ToplevelManager,
        _: <ToplevelManager as wayland_client::Proxy>::Event,
        _: &(),
        _: &wayland_client::Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
    }
}
