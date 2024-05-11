use std::{error, process::exit};

use x11rb::{
    connect,
    connection::Connection,
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
    pub fn new(connection: &RustConnection) -> Result<Self, Box<dyn error::Error>> {
        Ok(X11Atoms {
            _net_active_window: Self::get_atom(connection, "_NET_ACTIVE_WINDOW")?,
            _net_wm_pid: Self::get_atom(connection, "_NET_WM_PID")?,
            _net_wm_name: Self::get_atom(connection, "_NET_WM_NAME")?,
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
    connection: RustConnection,
    root: u32,
    pub atoms: X11Atoms, // atoms:
}

impl X11Helper {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let (connection, screen_num) = connect(None).unwrap_or_else(|_| {
            log_msg("Couldn't connect to X server", LogLevel::Error);
            exit(1);
        });
        log_msg("Connected to X creating x11 helper", LogLevel::Debug);

        let screen = &connection.setup().roots[screen_num];
        let root = screen.root;

        let atoms = X11Atoms::new(&connection)?;

        Ok(Self {
            atoms,
            connection,
            root,
        })
    }
    pub fn get_text_property(
        &self,
        win: u32,
        property: Atom,
        text_type: AtomEnum,
    ) -> Option<String> {
        let cookie =
            self.connection
                .get_property(false, win, property, text_type, 0, std::u32::MAX);

        let window_name = cookie.ok()?.reply().ok()?.value;

        Some(String::from_utf8(window_name).ok()?)
    }
    pub fn get_number_property(&self, win: u32, property: Atom) -> Option<u32> {
        let cookie =
            self.connection
                .get_property(false, win, property, AtomEnum::CARDINAL, 0, u32::MAX);

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
    pub fn get_root_window_id(&self, atom: Atom) -> Option<u32> {
        let cookie =
            self.connection
                .get_property(false, self.root, atom, AtomEnum::WINDOW, 0, u32::MAX);

        let window_id_reply = cookie.ok()?.reply().ok()?;
        let window_id = window_id_reply.value32()?.next()?;

        Some(window_id)
    }
}
