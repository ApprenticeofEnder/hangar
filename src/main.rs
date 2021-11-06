mod models;
mod view;

use std::io;
use requestty::Answers;

fn main() {
    let answers: Answers = view::hangar_create_menu();
    let hangar: models::Hangar = create_hangar(&answers).unwrap();
    hangar.preflight();
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

#[cfg(test)]
mod tests {
    use super::*;

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
}