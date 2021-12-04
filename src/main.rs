mod models;
mod view;

use home;
use requestty::Answers;

fn main() {
    let dir = home::home_dir().expect("Home directory not found!");
    let app_paths: view::InstallInfo = view::build_app_directories(dir.to_str().unwrap()); 
    let installed = view::check_install(&app_paths);
    let mut hangar: models::Hangar;
    if !installed {
        println!("Running first time setup...");
        view::install(&app_paths).expect("Installation failed!");
        println!("First time setup complete");
        let new_hangar_data = view::hangar_create_menu();
        match create_hangar(&new_hangar_data) {
            Ok(new_hangar) => {
                hangar = new_hangar;
                hangar_ctl(&mut hangar);
            },
            Err(reason) => {
                println!("{:?}", reason);
            }
        }
    }
    else {
        match view::hangar_load_menu(&app_paths) {
            Some(hangar_file) => { // Load an existing hangar
                // TODO: implement hangar loading functionality
                println!("{:?}", hangar_file);
            },
            None => { // Create a new hangar
                let new_hangar_data = view::hangar_create_menu();
                match create_hangar(&new_hangar_data) {
                    Ok(new_hangar) => {
                        hangar = new_hangar;
                        hangar_ctl(&mut hangar);
                    },
                    Err(reason) => {
                        println!("{:?}", reason);
                    }
                }
                
            }
        }
    }

}

fn hangar_ctl(hangar: &mut models::Hangar) {
    println!("{:?}", hangar)
}

fn create_hangar(data: &Answers) -> Result<models::Hangar, models::HangarCreateError>{
    match data.get("hangar_name") {
        Some(name) => match data.get("hangar_desc") {
            Some(desc) => Ok(models::Hangar {
                name: name.as_string().unwrap().to_string(),
                description: desc.as_string().unwrap().to_string(),
                flights: Vec::new()
            }),
            _ => Err(models::HangarCreateError::NoDescriptionGiven)
        },
        _ => Err(models::HangarCreateError::NoNameGiven)
    }
}

fn load_hangar(hangar_file: String) -> Result<models::Hangar, models::HangarCreateError>{
    Err(models::HangarCreateError::NoNameGiven)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_preflight() {
        let flight1 = models::Flight {
            program: String::from("pip"),
            preflight_args: vec![String::from("--version")],
            preflight_confirm: String::from("pip 21.0.1")
        };
        let mut hangar1 = models::Hangar {
            name: String::from("Test"),
            description: String::from("Test hangar"),
            flights: Vec::new() 
        };
        hangar1.flights.push(flight1);
        assert_eq!(hangar1.preflight(), true);
    }

    #[test]
    fn test_improper_preflight() {
        let flight1 = models::Flight {
            program: String::from("pip4"),
            preflight_args: vec![String::from("--version")],
            preflight_confirm: String::from("pip 21.0.1")
        };
        let mut hangar1 = models::Hangar {
            name: String::from("Test"),
            description: String::from("Test hangar"),
            flights: Vec::new() 
        };
        hangar1.flights.push(flight1);
        assert_eq!(hangar1.preflight(), false);
    }

    #[test]
    fn test_install() {
        let dir = home::home_dir().expect("Home directory not found!");
        let app_paths: view::InstallInfo = view::build_app_directories(dir.to_str().unwrap());
        fs::remove_dir_all(&app_paths.install).expect("Failure to delete data directory!");
        let installed = view::check_install(&app_paths);

        assert_eq!(installed, false);
        view::install(&app_paths).expect("Error installing program!");
    }
}