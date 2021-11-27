use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use requestty::{Question, Answer, Answers};
use titlecase::titlecase;

const APP_FOLDER: &str = ".hangar";
const DATA_FOLDER: &str = "data";

pub fn menu(){
    // TODO: Create the overall menu
}

/// Constructs app directory PathBufs for folders as needed.
/// Returns an InstallInfo struct
pub fn build_app_directories(home_dir: &str) -> InstallInfo {
    let install_path: PathBuf = [home_dir, APP_FOLDER].iter().collect();
    let data_path: PathBuf = [home_dir, APP_FOLDER, DATA_FOLDER].iter().collect();
    InstallInfo {
        install: install_path,
        data: data_path
    }
}


pub fn check_install(paths: &InstallInfo) -> bool{
    match fs::read_dir(&paths.install) {
        Ok(_) => match fs::read_dir(&paths.data) {
            Ok(_) => true,
            Err(_) => false
        },
        Err(_) => false
    }
}

pub fn install(paths: &InstallInfo) -> Result<bool, InstallError>{
    match fs::create_dir(&paths.install) {
        Ok(_) => create_data_path(paths),
        Err(reason) => {
            println!("Error creating {}: {}", paths.install.display(), reason);
            Err(InstallError::AppFolderCreateError)
        }
    }
}

fn create_data_path(paths: &InstallInfo) -> Result<bool, InstallError> {
    match fs::create_dir(&paths.data) {
        Ok(_) => Ok(true),
        Err(reason) => {
            println!("Error creating {}: {}", paths.data.display(), reason);
            Err(InstallError::DataFolderCreateError)
        }
    }
}

pub fn hangar_create_menu() -> Answers{
    let questions: Vec<Question> = vec![
        Question::input("hangar_name")
            .message("What is your new hangar's name")
            .build(),
        Question::input("hangar_desc")
            .message("What is the description of your new hangar")
            .default("Default Hangar.")
            .build()
    ];

    let answers: Answers = requestty::prompt(questions).unwrap();
    answers
}

pub fn hangar_load_menu(paths: &InstallInfo) -> String{
    let hangar_files = fs::read_dir(&paths.data).unwrap();
    let mut files: HashMap<String, String> = HashMap::new();
    let mut hangar_names: Vec<String> = Vec::new();
    for path in hangar_files {
        let path_string = path.unwrap().path().display().to_string();
        let hangar_name = get_hangar_name_from_file(&path_string);
        files.insert(hangar_name.clone(), path_string);
        hangar_names.push(hangar_name);
    }
    let question = Question::select("hangarfile")
        .message("Select Hangar to load from:")
        .choices(
            hangar_names
        ).build();
    let key = requestty::prompt_one(question).unwrap().try_into_list_item().unwrap().text;
    files.get(&key).unwrap().clone()
}

fn get_hangar_name_from_file(path: &String) -> String {
    let split_path: Vec<String> = path.split("/").map(|x| x.to_string()).collect();
    let file_name = split_path.last().unwrap();
    let hangar_name = titlecase(&file_name.replace("-"," ").replace(".json", ""));
    hangar_name
}

#[derive(Debug)]
pub struct InstallInfo{
    pub install: PathBuf,
    pub data: PathBuf
}

#[derive(Debug)]
pub enum InstallError {
    AppFolderCreateError,
    DataFolderCreateError
}

#[derive(Debug)]
pub enum LoadingOption {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_loading(){
        let tmp_path = "/tmp/sample-hangar.json";
        fs::File::create(tmp_path).expect("Error creating file!");
        let hangar_name: String = get_hangar_name_from_file(&String::from(tmp_path));
        assert_eq!(String::from("Sample Hangar"), hangar_name);
        fs::remove_file(tmp_path).unwrap();
    }
}