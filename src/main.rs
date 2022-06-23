mod utils;
mod static_files;

fn main() {
    // Declare project_name variable
    let mut project_name = String::new();

    println!("🌊 Creta project builder");

    let args = utils::Args::new();
    if args.len() < 2 {
        while project_name.len() < 1 {
            println!("What is the name of your project: ");
            if let Err(_) = std::io::stdin().read_line(&mut project_name) {
                println!("Cannot read your input");
                return;
            }
            project_name.pop();
            if project_name == "" {
                println!("Cannot set an empty name");
                println!();
            }
        }
    } else {
        // If project name is given
        project_name = match args.get(1) {
            // If command is init, set project name as current folder's name
            Some("init") => match utils::get_work_folder() {
                Some(s) => s,
                None => {
                    println!("Cannot build your project!");
                    return ;
                }
            },
            Some(p_name) => p_name.to_string(),
            None => {
                println!("Cannot build your project!");
                return
            }
        };
    }

    // Define project_directory
    let path_to_project = utils::build_path(&project_name);

    // Create the main folder and src folder
    // If an error occurs stop the app
    match std::fs::create_dir_all(format!("{}src", path_to_project)) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }
    
    // Create and write the files
    match std::fs::write(format!("{}package.json", path_to_project), static_files::PACKAGE_JSON.replace("PROJECT_NAME", &project_name)) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }
    match std::fs::write(format!("{}tsconfig.json", path_to_project), static_files::TSCONFIG_JSON) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }
    match std::fs::write(format!("{}.gitignore", path_to_project), static_files::_GITIGNORE) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }
    match std::fs::write(format!("{}src/index.ts", path_to_project), static_files::SRC_INDEX_TS) {
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
