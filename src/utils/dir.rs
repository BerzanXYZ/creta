// get_work_dir() gets the current working directory
// Returns Some(String) or None
pub fn get_work_dir() -> Option<String> {
    match std::env::current_dir() {
        Ok(path_buf) => Some(path_buf.to_str().unwrap().to_string()),
        Err(_) => None,
    }
}

pub fn get_work_folder() -> Option<String> {
    match get_work_dir() {
        Some(wd) => match wd.split("/").last() {
            Some(dir) => Some(dir.to_string()),
            None => None
        },
        None => None,
    }
}

pub fn build_path(pro_name: &str, cmd: &str) -> String {
    let cur_dir = match get_work_dir() {
        Some(cd) => cd,
        None => panic!(),
    };
    if cmd == "init" {
        format!("{}/", cur_dir)
    } else {
        format!("{}/{}/", cur_dir, pro_name)
    }
}