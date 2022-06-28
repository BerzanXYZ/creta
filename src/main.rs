mod utils;
mod static_files;

fn main() {
    let mut project_name = String::new();
    let mut command = String::new();

    println!("🌊 Creta app builder");

    let args = utils::Args::new();
    // If no command is given;
    if args.len() < 2 {
        // While project_name is empty
        while project_name.len() < 1 {
            println!("What is the name of your project: ");
            // Read user input and set project_name
            if let Err(_) = std::io::stdin().read_line(&mut project_name) {
                println!("Cannot read your input");
                return;
            }
            // Delete newline character
            project_name.pop();
            // If project name is empty, tell user
            if project_name == "" {
                println!("Cannot set an empty name");
                println!();
            }
        }
        // If a command name is given, do these
    } else {
        command = match args.get(1) {
            // If the command is init, set project name as current folder's name
            Some("init") => {
                project_name = utils::get_work_folder().unwrap();
                "init".to_string()
            },
            // If the command represents project name, set it
            Some(p_name) => {
                project_name = p_name.to_string();
                "".to_string()
            },
            None => {
                println!("Cannot build your project!");
                return
            }
        };
    }
    

    // Define project_directory
    let path_to_project = utils::build_path(&project_name, &command);

    // Create the main folder and src folder
    // If an error occurs stop the app
    match std::fs::create_dir_all(format!("{}src", path_to_project)) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }
    
    // Create and write the files
    match std::fs::write(format!("{}package.json", path_to_project), static_files::PACKAGE_JSON.replace("project_name", &project_name)) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }
    match std::fs::write(format!("{}tsconfig.json", path_to_project), static_files::TSCONFIG_JSON) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }
    match std::fs::write(format!("{}yarn.lock", path_to_project), static_files::YARN_LOCK) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }
    match std::fs::write(format!("{}.gitignore", path_to_project), static_files::GITIGNORE) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }
    match std::fs::write(format!("{}src/index.ts", path_to_project), static_files::SRC_INDEX_TS) {
        Ok(_) => (),
        Err(_) => {println!("🌊 An error occured while creating the app..."); return;},
    }

    println!();

    // Install node_modules by using this command
    if command == "init" {
        if let Err(_) = std::process::Command::new("yarn").output() {
            println!("Yarn isn't installed...");
        }
    } else {
        if let Err(_) = std::process::Command::new("yarn").current_dir(&project_name).output() {
            println!("Yarn isn't installed...");
        }
    }


    println!("To start your app:");
    // If current directory is in the project's base folder
    // User doesn't have to use "cd" command
    if command != "init" {
        println!("   cd {}", project_name);
    }
    println!("   yarn start");
    println!();

}
