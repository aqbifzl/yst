pub struct ActiveWinProperties {
    pub name: Option<String>,
    pub cmd: Option<String>,
}

impl ActiveWinProperties {
    pub fn new() -> Self {
        Self {
            name: None,
            cmd: None,
        }
    }
    pub fn from(name: &str, cmd: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            cmd: Some(cmd.to_string()),
        }
    }
}
