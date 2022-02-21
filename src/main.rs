mod models;
mod view;

use requestty::Answers;
use std::collections::HashMap;

#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    let dir = home::home_dir().expect("Home directory not found!");
    let app_paths: view::InstallInfo = view::build_app_directories(dir.to_str().unwrap());
    let installed = view::check_install(&app_paths);
    let mut hangar: models::Hangar;
    if !installed {
        println!("Running first time setup...");
        info!("{}", "Running first time setup...");
        view::install(&app_paths).expect("Installation failed!");
        println!("First time setup complete");
        info!("{}", "First time setup complete");
        let new_hangar_data = view::hangar_create_menu();
        match create_hangar(&new_hangar_data) {
            Ok(new_hangar) => {
                info!("{}", "Hangar created successfully");
                hangar = new_hangar;
                hangar_ctl(&mut hangar);
            }
            Err(reason) => {
                error!("{:?}", reason);
            }
        }
    } else {
        match view::hangar_load_menu(&app_paths) {
            Some(hangar_file) => {
                // Load an existing hangar
                // TODO: implement hangar loading functionality
                info!("{:?}", hangar_file);
            }
            None => {
                // Create a new hangar
                let new_hangar_data = view::hangar_create_menu();
                match create_hangar(&new_hangar_data) {
                    Ok(new_hangar) => {
                        hangar = new_hangar;
                        hangar_ctl(&mut hangar);
                    }
                    Err(reason) => {
                        error!("{:?}", reason);
                    }
                }
            }
        }
    }
}

fn hangar_ctl(hangar: &mut models::Hangar) {
    loop {
        match view::menu() {
            Ok(view::MenuAction::Preflight) => {
                // Preflight code
            }
            Ok(view::MenuAction::ManageFlights) => {
                flight_management(hangar);
            }
            Ok(view::MenuAction::Exit) => {
                break;
            }
            Err(reason) => match reason {
                Some(requestty_error) => {
                    error!("{:?}", requestty_error);
                }
                None => {
                    error!("An option was given that doesn't exist.");
                }
            },
        }
    }
}

fn flight_management(hangar: &mut models::Hangar) {
    loop {
        let mut flight_names: Vec<String> = Vec::new();
        for flight in &hangar.flights {
            flight_names.push(flight.name.clone());
        }
        flight_names.push("Create New Flight".to_string());
        flight_names.push("Exit".to_string());
        let flight_index = view::flights_menu(&flight_names);
        let option_count = flight_names.len() as i32;
        let create_index = option_count - 2;
        let exit_index = option_count - 1;
        match flight_index {
            flight_index if flight_index == create_index => {
                let answers = view::flight_create_menu();
                match create_flight(&answers) {
                    Ok(flight) => {
                        hangar.flights.push(flight);
                    }
                    Err(models::FlightCreateError::InvalidParameter) => {
                        error!("{}", "Invalid parameter given");
                    }
                }
            }
            flight_index if flight_index == exit_index => {
                break;
            }
            -1 => {
                error!("{}", "Error, aborting");
                break;
            }
            _ => {
                info!("{}", "Modifying existing flight!");
            }
        }
    }
}

fn create_hangar(data: &Answers) -> Result<models::Hangar, models::HangarCreateError> {
    match data.get("hangar_name") {
        Some(name) => match data.get("hangar_desc") {
            Some(desc) => Ok(models::Hangar {
                name: name.as_string().unwrap().to_string(),
                description: desc.as_string().unwrap().to_string(),
                flights: Vec::new(),
            }),
            _ => Err(models::HangarCreateError::NoDescriptionGiven),
        },
        _ => Err(models::HangarCreateError::NoNameGiven),
    }
}

fn load_hangar(hangar_file: String) -> Result<models::Hangar, models::HangarCreateError> {
    Err(models::HangarCreateError::NoNameGiven)
}

fn create_flight(data: &Answers) -> Result<models::Flight, models::FlightCreateError> {
    let mut store: HashMap<String, String> = HashMap::new();
    for (key, value) in data.clone().into_iter() {
        if value.is_string() {
            store.insert(key, value.try_into_string().unwrap());
        } else if value.is_list_item() {
            store.insert(key, value.try_into_list_item().unwrap().text);
        }
    }
    match models::Flight::new(store) {
        Some(flight) => Ok(flight),
        _ => Err(models::FlightCreateError::InvalidParameter),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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
