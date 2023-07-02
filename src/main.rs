extern crate  ini;
use std::process::{Command, self};
use std::{env, fs, io};
use std::path::Path;
use ini::Ini;

#[derive(std::fmt::Debug)]
struct ApplicationEntry {
    filepath: String,
    applicationName: String,
    command: String
}

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

fn get_appliactions(list_of_dirs: &Vec<String>) -> Vec<ApplicationEntry>{
    let mut temp: Vec<ApplicationEntry> = Vec::new();
    for dir in list_of_dirs {
        let files = fs::read_dir(&dir).unwrap();
        println!("Directory: {}", &dir);
        for file in files {
            let filePath = file.unwrap().path().into_os_string().into_string().unwrap();
            let conf = Ini::load_from_file(
                &filePath
            ).unwrap();
            let section = &conf.section(Some("Desktop Entry"));
            if(section.is_some()){
                let appName = section.unwrap().get("Name").unwrap_or("");
                let appCommand = section.unwrap().get("Exec").unwrap_or("");
                let newApp: ApplicationEntry = ApplicationEntry {
                    filepath: filePath.to_string(), 
                    applicationName: String::from(appName), 
                    command: String::from(appCommand)
                };
                temp.push(newApp);
            }
            //temp.push(&file.unwrap().path().to_str().unwrap());
        }
    }
    return temp;
}

fn main() {
    let list_of_dirs = get_desktop_dirs();
    let mut applications = get_appliactions(&list_of_dirs);
    //println!("{:#?}", &list_of_dirs);
    println!("{:#?}", &applications);
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input);
    user_input = user_input.trim().to_owned();
    let mut executeCommand : Vec<ApplicationEntry>  = applications
        .into_iter()
        .filter(|a| a.applicationName == user_input)
        .collect();
    println!("{:?}", executeCommand);
    Command::new(&mut executeCommand.pop().unwrap().command)
    .stdout(process::Stdio::null())
    .stderr(process::Stdio::null())
    .spawn();
    //println!("Command to be executed: {}", executeCommand);
}
