use std::thread::spawn;

use wayland_client::{protocol::wl_registry, Connection, Dispatch};

use crate::wl_client::toplevel::zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1;

pub mod toplevel_handler;

pub struct YstState;

impl Dispatch<wl_registry::WlRegistry, ()> for YstState {
    fn event(
        _: &mut Self,
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
            if interface == "zwlr_foreign_toplevel_manager_v1" {
                proxy.bind::<ZwlrForeignToplevelManagerV1, _, _>(name, version, qhandle, ());
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

        loop {
            queue.blocking_dispatch(&mut YstState).unwrap();
        }
    });
}
