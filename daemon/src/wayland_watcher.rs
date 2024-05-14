use std::thread::spawn;

use shared::config::MAX_IDLE_SECONDS;
use wayland_client::{
    protocol::{
        wl_registry,
        wl_seat::{self, WlSeat},
    },
    Connection, Dispatch, QueueHandle,
};

use crate::wl_client::{
    idle::ext_idle_notifier_v1::ExtIdleNotifierV1,
    toplevel::zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1,
};

pub mod idle_handler;
pub mod toplevel_handler;

pub struct YstState {
    idle_interface: Option<ExtIdleNotifierV1>,
}

impl YstState {
    fn new() -> Self {
        Self {
            idle_interface: None,
        }
    }
}

impl Dispatch<WlSeat, ()> for YstState {
    fn event(
        state: &mut Self,
        seat: &WlSeat,
        _: wl_seat::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let Some(idle_interface) = &state.idle_interface {
            idle_interface.get_idle_notification(MAX_IDLE_SECONDS as u32 * 1000, seat, qh, ());
        }
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for YstState {
    fn event(
        state: &mut Self,
        proxy: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match &interface[..] {
                "zwlr_foreign_toplevel_manager_v1" => {
                    proxy.bind::<ZwlrForeignToplevelManagerV1, _, _>(name, version, qhandle, ());
                }
                "ext_idle_notifier_v1" => {
                    if state.idle_interface.is_none() {
                        state.idle_interface =
                            Some(proxy.bind::<ExtIdleNotifierV1, _, _>(name, version, qhandle, ()));
                    }
                }
                "wl_seat" => {
                    proxy.bind::<WlSeat, _, _>(name, version, qhandle, ());
                }
                _ => (),
            }
        }
    }
}

pub fn handle_wayland() {
    spawn(|| {
        let connection =
            Connection::connect_to_env().expect("Connection to wayland with env variable failed");
        let dpy = connection.display();

        let mut queue = connection.new_event_queue();
        let qh = queue.handle();
        dpy.get_registry(&qh, ());

        let mut yst_state = YstState::new();

        loop {
            queue.blocking_dispatch(&mut yst_state).unwrap();
        }
    });
}
