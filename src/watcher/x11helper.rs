use std::{
    error,
    sync::{Arc, Mutex},
};

use x11rb::{
    protocol::xproto::{Atom, AtomEnum, ConnectionExt},
    rust_connection::RustConnection,
};

use crate::utils::logger::{log_msg, LogLevel};

pub struct X11Atoms {
    pub _net_active_window: Atom,
    pub _net_wm_pid: Atom,
    pub _net_wm_name: Atom,
}

impl X11Atoms {
    pub fn new(connection: Arc<Mutex<RustConnection>>) -> Result<Self, Box<dyn error::Error>> {
        let connection = connection.lock().unwrap();

        Ok(X11Atoms {
            _net_active_window: Self::get_atom(&connection, "_NET_ACTIVE_WINDOW")?,
            _net_wm_pid: Self::get_atom(&connection, "_NET_WM_PID")?,
            _net_wm_name: Self::get_atom(&connection, "_NET_WM_NAME")?,
        })
    }
    pub fn get_atom(
        connection: &RustConnection,
        name: &str,
    ) -> Result<Atom, Box<dyn error::Error>> {
        Ok(connection
            .intern_atom(false, name.as_bytes())?
            .reply()?
            .atom)
    }
}

pub struct X11Helper {
    connection: Arc<Mutex<RustConnection>>,
    pub atoms: X11Atoms, // atoms:
}

impl X11Helper {
    pub fn new(connection: Arc<Mutex<RustConnection>>) -> Result<Self, Box<dyn error::Error>> {
        let atoms = X11Atoms::new(connection.clone())?;

        Ok(Self {
            atoms,
            connection: connection.clone(),
        })
    }
    pub fn get_text_property(
        &self,
        win: u32,
        property: Atom,
        text_type: AtomEnum,
    ) -> Option<String> {
        let connection = self.connection.lock().unwrap();
        let cookie = connection.get_property(false, win, property, text_type, 0, std::u32::MAX);

        let window_name = cookie.ok()?.reply().ok()?.value;

        Some(String::from_utf8(window_name).ok()?)
    }
    pub fn get_number_property(&self, win: u32, property: Atom) -> Option<u32> {
        let connection = self.connection.lock().unwrap();
        let cookie = connection.get_property(false, win, property, AtomEnum::CARDINAL, 0, u32::MAX);

        let reply = cookie.ok()?.reply().ok()?.value32()?.next()?;

        Some(reply)
    }
    pub fn get_window_class_name(&self, win: u32) -> Option<String> {
        Some(
            self.get_text_property(win, AtomEnum::WM_CLASS.into(), AtomEnum::STRING)?
                .split('\0')
                .nth(1)?
                .to_string(),
        )
    }
    pub fn get_window_net_name(&self, win: u32) -> Option<String> {
        self.get_text_property(win, self.atoms._net_wm_name, AtomEnum::ANY)
    }
    pub fn get_window_wm_name(&self, win: u32) -> Option<String> {
        self.get_text_property(win, AtomEnum::WM_NAME.into(), AtomEnum::STRING)
    }
    pub fn get_window_id_from_atom(&self, win: u32, atom: Atom) -> Option<u32> {
        let connection = self.connection.lock().unwrap();
        let cookie = connection.get_property(false, win, atom, AtomEnum::WINDOW, 0, u32::MAX);

        let window_id_reply = cookie.ok()?.reply().ok()?;
        let window_id = window_id_reply.value32()?.next()?;

        Some(window_id)
    }
}
