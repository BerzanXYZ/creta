mod utils;
mod static_files;

fn main() {
    // Declare project_name variable
    let mut project_name = String::new();

    // Check if enough args are given
    // If not stop the app
    if std::env::args().count() == 2 {
        project_name = match std::env::args().nth(1) {
            // Convert the project name to lowercase to respect NPM naming rules
            Some(pn) => pn.to_lowercase(),
            None => {println!("🌊 An error occured while setting project name..."); return;},
        };
    } else {
        // If the user didn't set a project name
        // Tell user what to do, then stop the app
        println!("🌊 Try to run Creta by using the command below:");
        println!("creta my-project");
        return;
    }


    // Get and assign work directory
    // If an error occurs, stop the app
    let work_dir = match utils::get_work_dir() {
        Some(wd) => wd,
        None => {println!("🌊 An error occured while reading work directory..."); return;},
    };

    // Define project_directory
    let project_dir = utils::build_pro_dir(&work_dir, &project_name);

    
    println!();
    println!("🌊 Creta builds your app...");


}
