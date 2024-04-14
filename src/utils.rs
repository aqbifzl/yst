pub mod fs;
pub mod logger;
pub mod process;

pub fn get_current_date() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

pub fn get_current_time() -> String {
    chrono::Local::now().format("%H:%M:%S").to_string()
}

pub fn get_current_date_and_time() -> String {
    get_current_date() + " " + &get_current_time()
}
