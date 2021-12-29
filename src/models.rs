use std::process::Command;

#[derive(Debug)]
pub struct Flight {
    pub program: String,
    pub preflight_args: Vec<String>,
    pub preflight_confirm: String,
}

impl Flight {
    pub fn preflight(&self) -> bool {
        println!("Running test for {}", self.program);
        let mut preflight_command = Command::new(&self.program);
        for arg in &self.preflight_args {
            preflight_command.arg(&arg);
        }
        match preflight_command.output() {
            Ok(result) => {
                let output = String::from_utf8(result.stdout).unwrap();
                let errors = String::from_utf8(result.stderr).unwrap();
                self.check_preflight_result(output, errors)
            }
            Err(error) => {
                println!("Problem executing program {}: {}", &self.program, error);
                false
            }
        }
    }

    fn check_preflight_result(&self, output: String, errors: String) -> bool {
        if !errors.is_empty() {
            println!("Test failed!");
            return false;
        }
        println!("Test ran without errors!");
        if !output.contains(&self.preflight_confirm) {
            println!("Test did not contain expected output!");
            return false;
        }
        println!("Test complete!");
        true
    }
}

pub fn new_flight(
    program: &mut String,
    preflight_args: &[String],
    preflight_confirm: &mut String,
) -> Flight {
    Flight {
        program: program.trim().to_string(),
        preflight_args: preflight_args.to_owned(),
        preflight_confirm: preflight_confirm.trim().to_string(),
    }
}

#[derive(Debug)]
pub struct Hangar {
    pub name: String,
    pub description: String,
    pub flights: Vec<Flight>,
}

impl Hangar {
    pub fn preflight(&self) -> bool {
        for flight in &(self.flights) {
            if !flight.preflight() {
                return false;
            }
        }
        println!("Preflight Checklist Complete!");
        true
    }
}

#[derive(Debug)]
pub enum HangarCreateError {
    NoNameGiven,
    NoDescriptionGiven,
    InvalidFile,
}
