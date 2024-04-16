use std::{error::Error, path::PathBuf};

pub fn get_current_time() -> String {
    chrono::Local::now().format("%H:%M:%S").to_string()
}

pub fn get_specific_date(offset: i32) -> String {
    (chrono::Local::now() + chrono::Duration::days(offset.into()))
        .format("%Y-%m-%d")
        .to_string()
}

pub fn get_current_date_and_time() -> String {
    get_specific_date(0) + " " + &get_current_time()
}

pub fn ms_to_human_readable_time_span(mut ms: u128) -> String {
    let h = ms / (1000 * 60 * 60);
    ms %= 1000 * 60 * 60;

    let m = ms / (1000 * 60);
    ms %= 1000 * 60;

    let s = ms / 1000;

    let mut res = String::new();

    if h > 0 {
        res.push_str(&format!("{}h ", h));
    }
    if m > 0 {
        res.push_str(&format!("{}m ", m));
    }
    if s > 0 {
        res.push_str(&format!("{}s ", s));
    }

    if res.is_empty() {
        res = "<0ms".to_string()
    }

    res
}

pub fn escape_home_dir(str: &str) -> Result<PathBuf, Box<dyn Error>> {
    match str.find('~') {
        Some(index) => {
            if index != 0 {
                return Ok(PathBuf::from(str));
            }

            let str: String = str.to_string();

            let homedir = env!("HOME");

            Ok(PathBuf::from(str.replacen('~', homedir, 1)))
        }
        None => Ok(PathBuf::from(str)),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::utils::escape_home_dir;

    #[test]
    fn test_escape_home_dir() {
        let res = escape_home_dir("/home/user/.file").unwrap();
        assert_eq!(res, PathBuf::from("/home/user/.file"));

        let res = escape_home_dir("/home/user").unwrap();
        assert_eq!(res, PathBuf::from("/home/user"));

        let res = escape_home_dir("/home/user///").unwrap();
        assert_eq!(res, PathBuf::from("/home/user///"));

        let res = escape_home_dir("").unwrap();
        assert_eq!(res, PathBuf::from(""));

        let res = escape_home_dir("/").unwrap();
        assert_eq!(res, PathBuf::from("/"));

        let res = escape_home_dir("~").unwrap();
        assert_eq!(res, PathBuf::from(env!("HOME")));

        let res = escape_home_dir("~/").unwrap();
        assert_eq!(res, PathBuf::from(env!("HOME")));

        let res = escape_home_dir("~/test/~test").unwrap();
        assert_eq!(res, PathBuf::from(env!("HOME").to_string() + "/test/~test"));

        let res = escape_home_dir("/home/user/~/my~log~file.~").unwrap();
        assert_eq!(res, PathBuf::from("/home/user/~/my~log~file.~"));
    }
}
