use requestty::{Answers, Question};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use titlecase::titlecase;

const APP_FOLDER: &str = ".hangar";
const DATA_FOLDER: &str = "data";

pub fn menu() -> Result<MenuAction, Option<requestty::ErrorKind>> {
    let action_schema: Vec<(String, MenuAction)> = vec![
        ("Preflight".to_string(), MenuAction::Preflight),
        ("Manage Flights".to_string(), MenuAction::ManageFlights),
        ("Exit".to_string(), MenuAction::Exit),
    ];
    let mut actions: HashMap<String, MenuAction> = HashMap::new();
    let options: Vec<String> = action_schema
        .into_iter()
        .map(|x| {
            actions.insert(x.0.clone(), x.1);
            x.0
        })
        .collect();
    let menu = Question::select("option")
        .message("Select an option")
        .choices(options)
        .build();

    match requestty::prompt_one(menu) {
        Ok(answer) => {
            let option = answer.try_into_list_item().unwrap().text;
            match actions.get(&option) {
                Some(action) => Ok(*action),
                None => Err(None),
            }
        }
        Err(reason) => Err(Some(reason)),
    }
}

/// Constructs app directory PathBufs for folders as needed.
/// Returns an InstallInfo struct
pub fn build_app_directories(home_dir: &str) -> InstallInfo {
    let install_path: PathBuf = [home_dir, APP_FOLDER].iter().collect();
    let data_path: PathBuf = [home_dir, APP_FOLDER, DATA_FOLDER].iter().collect();
    InstallInfo {
        install: install_path,
        data: data_path,
    }
}

pub fn check_install(paths: &InstallInfo) -> bool {
    match fs::read_dir(&paths.install) {
        Ok(_) => fs::read_dir(&paths.data).is_ok(),
        Err(_) => false,
    }
}

pub fn install(paths: &InstallInfo) -> Result<bool, InstallError> {
    match fs::create_dir(&paths.install) {
        Ok(_) => create_data_path(paths),
        Err(reason) => {
            error!("Error creating {}: {}", paths.install.display(), reason);
            Err(InstallError::AppFolderCreateError)
        }
    }
}

fn create_data_path(paths: &InstallInfo) -> Result<bool, InstallError> {
    match fs::create_dir(&paths.data) {
        Ok(_) => Ok(true),
        Err(reason) => {
            error!("Error creating {}: {}", paths.data.display(), reason);
            Err(InstallError::DataFolderCreateError)
        }
    }
}

pub fn hangar_create_menu() -> Answers {
    let questions: Vec<Question> = vec![
        Question::input("hangar_name")
            .message("What is your new hangar's name")
            .build(),
        Question::input("hangar_desc")
            .message("What is the description of your new hangar")
            .default("Default Hangar.")
            .build(),
    ];

    let answers: Answers = requestty::prompt(questions).unwrap();
    answers
}

pub fn hangar_load_menu(paths: &InstallInfo) -> Option<PathBuf> {
    let hangar_files = fs::read_dir(&paths.data).unwrap();
    let mut files: HashMap<String, PathBuf> = HashMap::new();
    let mut options: Vec<String> = Vec::new();
    let create_option: String = String::from("Create New Hangar");
    for path in hangar_files {
        let path_buf = path.unwrap().path();
        let path_string = path_buf.display().to_string();
        let hangar_name = get_hangar_name_from_file(&path_string);
        files.insert(hangar_name.clone(), path_buf);
        options.push(hangar_name);
    }
    options.push(create_option.clone());
    let question: Question = Question::select("hangarfile")
        .message("Select Hangar to load from:")
        .choices(options)
        .build();
    match requestty::prompt_one(question) {
        Ok(answer) => {
            let key = answer.try_into_list_item().unwrap().text;
            if key.eq(&create_option) {
                return None;
            }
            Some(files.get(&key).unwrap().clone())
        }
        Err(reason) => {
            error!("{:?}", reason);
            None
        }
    }
}

pub fn flights_menu(flight_names: &[String]) -> i32 {
    let question: Question = Question::select("flight")
        .message("Select a Flight:")
        .choices(flight_names)
        .build();
    match requestty::prompt_one(question) {
        Ok(answer) => {
            let key = answer.try_into_list_item().unwrap().text;
            let option_count: i32 = flight_names.len() as i32;
            let filtered: Vec<i32> = (0..option_count)
                .into_iter()
                .filter(|x| flight_names.get(*x as usize).unwrap().eq(&key))
                .collect();
            return *filtered.first().unwrap();
        }
        Err(reason) => {
            error!("{:?}", reason);
            -1
        }
    }
}

pub fn flight_create_menu() -> Answers {
    let questions: Vec<Question> = vec![
        Question::input("flight_name")
            .message("What is your new flight's name")
            .build(),
        Question::input("flight_exec")
            .message("What is the executable for the new flight")
            .build(),
        Question::input("preflight_args")
            .message("Add comma-separated arguments for the preflight check (e.g. -c,echo hello)")
            .build(),
        Question::select("stream")
            .message("Which output stream do you wish to check for preflight?")
            .choices(vec![
                "stdout",
                "stderr"
            ])
            .build(),
        Question::input("package_name")
            .message("Enter the package name as it is used in apt or apt-get (leave blank for none):")
            .build(),
        Question::input("reinstall")
            .message("Enter a reinstall command to use in the event of program failure:")
            .default("apt-get --reinstall install {package_name}")
            .build()
    ];

    let answers: Answers = requestty::prompt(questions).unwrap();
    answers
}

fn get_hangar_name_from_file(path: &str) -> String {
    let split_path: Vec<String> = path.split('/').map(|x| x.to_string()).collect();
    let file_name = split_path.last().unwrap();
    titlecase(&file_name.replace("-", " ").replace(".json", ""))
}

#[derive(Debug)]
pub struct InstallInfo {
    pub install: PathBuf,
    pub data: PathBuf,
}

#[derive(Debug)]
pub enum InstallError {
    AppFolderCreateError,
    DataFolderCreateError,
}

#[derive(Clone, Copy, Debug)]
pub enum MenuAction {
    Exit,
    ManageFlights,
    Preflight,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_loading() {
        let tmp_path = "/tmp/sample-hangar.json";
        fs::File::create(tmp_path).expect("Error creating file!");
        let hangar_name: String = get_hangar_name_from_file(&String::from(tmp_path));
        assert_eq!(String::from("Sample Hangar"), hangar_name);
        fs::remove_file(tmp_path).unwrap();
    }
}
