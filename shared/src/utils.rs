use std::{error::Error, path::PathBuf};

pub fn get_current_date() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

pub fn get_current_time() -> String {
    chrono::Local::now().format("%H:%M:%S").to_string()
}

pub fn get_current_date_and_time() -> String {
    get_current_date() + " " + &get_current_time()
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
