use std::fs::read_to_string;

const PROC_PATH: &str = "/proc/";

pub fn get_name_by_pid(pid: u32) -> Option<String> {
    let pid_str = pid.to_string();
    let process_cmdline_path = format!("{}{}/comm", PROC_PATH, pid_str);
    let content = read_to_string(process_cmdline_path).ok()?;
    let content = content.trim();

    if !content.is_empty() {
        return Some(content.to_string());
    }

    None
}
