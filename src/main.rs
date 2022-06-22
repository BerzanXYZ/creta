mod utils;
mod static_files;

fn main() {
    // Declare project_name variable
    let project_name;

    // Check if enough args are given
    // If not stop the app
    if std::env::args().count() == 2 {
        project_name = match std::env::args().nth(1) {
            // Convert the project name to lowercase to respect NPM naming rules
            Some(pn) => pn.to_lowercase(),
            None => {println!("🌊 An error occured while setting project name..."); return;},
        }
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
    let project_dir = format!("{}/{}/",work_dir, project_name);

    
    println!();
    println!("🌊 Creta builds your app...");

    // Create the main folder and src folder
    // If an error occurs stop the app
    match std::fs::create_dir_all(format!("{}src", project_dir)) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }
    
    // Create and write the files
    match std::fs::write(format!("{}package.json", project_dir), static_files::PACKAGE_JSON.replace("PROJECT_NAME", &project_name)) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }
    match std::fs::write(format!("{}tsconfig.json", project_dir), static_files::TSCONFIG_JSON) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }
    match std::fs::write(format!("{}.gitignore", project_dir), static_files::_GITIGNORE) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }
    match std::fs::write(format!("{}src/index.ts", project_dir), static_files::SRC_INDEX_TS) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }

    // Print success message
    println!();
    println!("All done!");
    println!();
    println!("To prepare your app, run the commands below:");
    println!("  cd {}", project_name);
    println!("  yarn install");
    println!();
    println!("To start your app, run the commands below:");
    println!("  yarn start");
    println!();
    println!("🌊 By Creta");

}
