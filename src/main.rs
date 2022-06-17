mod utils;
mod static_files;

fn main() {
    // Get and assign work directory
    // If an error occurs, stop the app
    let work_dir = match utils::get_work_dir() {
        Some(wd) => wd,
        None => {println!("❌ An error occured while reading work directory..."); return;},
    }; 
    
    println!();
    println!("🌊 Creta, app builder");
    println!("Work Directory is {}", work_dir);
    println!("{}", static_files::PACKAGE_JSON);
    println!("{}", static_files::TSCONFIG_JSON);
    println!("{}", static_files::_GITIGNORE);
    println!("{}", static_files::SRC__INDEX_TS);
}
