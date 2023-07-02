extern crate  ini;
use std::process::{Command, self};
use std::{env, fs, io};
use std::path::Path;
use ini::Ini;


/// Struct store application infomation
#[derive(std::fmt::Debug)]
struct ApplicationEntry {
    /// Path file of execution file
    filepath: String,
    /// Name of application
    application_name: String,
    /// Command to run application
    command: String
}

/// Gets all .desktop files in a system
fn get_desktop_dirs() -> Vec<String>{
    let mut dir_vec : Vec<String> = Vec::new();

    if env::var("HOME").is_ok() {
        let local_desktop_files = env::var("HOME").unwrap() + "/.local/share/applications";
        if Path::new(&local_desktop_files).exists() {
            dir_vec.push(local_desktop_files);
        }
        if Path::new("/usr/share/applications/").exists() {
            dir_vec.push("/usr/share/applications/".to_owned());
        }
        if Path::new("/usr/local/share/applications/").exists() {
            dir_vec.push("/usr/local/share/applications/".to_owned());
        }
    }
    return dir_vec;
}

/// Converts .desktop files into structs
fn get_applications(list_of_dirs: &Vec<String>) -> Vec<ApplicationEntry>{
    let mut temp: Vec<ApplicationEntry> = Vec::new();
    for dir in list_of_dirs {
        let files = fs::read_dir(&dir);
        if files.is_ok(){
            for file in files.unwrap() {
                if file.is_ok() {
                    let file_path = file.unwrap().path().into_os_string().into_string().unwrap();
                    let conf = Ini::load_from_file(
                        &file_path
                    );
                    if conf.is_ok() {
                        let conf_unwrap = conf.unwrap();
                        let section = &conf_unwrap.section(Some("Desktop Entry"));
                        if section.is_some() {
                            let app_name = section.unwrap().get("Name").unwrap_or("");
                            let app_command = section.unwrap().get("Exec").unwrap_or("");
                            let new_app: ApplicationEntry = ApplicationEntry {
                                filepath: file_path.to_string(), 
                                application_name: String::from(app_name), 
                                command: String::from(app_command)
                            };
                            temp.push(new_app);
                        }
                    }
                }
            }
        }
    }
    return temp;
}

fn main() {
    let list_of_dirs = get_desktop_dirs();
    let applications = get_applications(&list_of_dirs);
    println!("{:#?}", &applications);
    
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input);
    // Removes new line character at the end of input
    user_input = user_input.trim().to_owned();

    // Filters for specific application entry
    let mut execute_command : Vec<ApplicationEntry>  = applications
        .into_iter()
        .filter(|a| a.application_name == user_input)
        .collect();
    println!("{:?}", execute_command);

    // Executes command (We set the output and error to /dev/null so we are not waiting on the program's output)
    Command::new(&mut execute_command.pop().unwrap().command)
    .stdout(process::Stdio::null())
    .stderr(process::Stdio::null())
    .spawn();
}
