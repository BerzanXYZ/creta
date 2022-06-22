// get_work_dir() gets the current working directory
// Returns Some(String) or None
pub fn get_work_dir() -> Option<String> {
    match std::env::current_dir() {
        Ok(path_buf) => Some(path_buf.to_str().unwrap().to_string()),
        Err(_) => None,
    }
}