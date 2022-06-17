mod utils;
mod static_files;
use utils::get_work_dir;

fn main() {
    // Get and assign work directory
    // If an error occurs, stop the app
    let work_dir = match get_work_dir() {
        Some(wd) => wd,
        None => {println!("❌ An error occured while reading work directory..."); return;},
    }; 
    
    println!();
    println!("🌊 Creta, build TypeScript apps...");
    println!("Work Directory is {}", work_dir);
    println!("{}", crate::static_files::PACKAGE_JSON);
}
