use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use wayland_client::Dispatch;

use crate::wl_client::idle::{
    ext_idle_notification_v1::{self, ExtIdleNotificationV1},
    ext_idle_notifier_v1::ExtIdleNotifierV1,
};

use ext_idle_notification_v1::Event as IdleNotificationEvent;

use super::YstState;

lazy_static! {
    pub static ref IS_IDLE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

pub fn get_idle_state() -> bool {
    IS_IDLE.lock().unwrap().clone()
}

impl Dispatch<ExtIdleNotificationV1, ()> for YstState {
    fn event(
        _: &mut Self,
        _: &ExtIdleNotificationV1,
        event: <ExtIdleNotificationV1 as wayland_client::Proxy>::Event,
        _: &(),
        _: &wayland_client::Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            IdleNotificationEvent::Idled => {
                *IS_IDLE.lock().unwrap() = true;
            }
            IdleNotificationEvent::Resumed => {
                *IS_IDLE.lock().unwrap() = false;
            }
        }
    }
}

impl Dispatch<ExtIdleNotifierV1, ()> for YstState {
    fn event(
        _: &mut Self,
        _: &ExtIdleNotifierV1,
        _: <ExtIdleNotifierV1 as wayland_client::Proxy>::Event,
        _: &(),
        _: &wayland_client::Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
    }
}
