use std::process::Command;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Flight {
    pub name: String,
    pub flight_exec: String,
    pub preflight_args: Vec<String>,
    pub stream: String,
    pub flag: String,
    pub package_name: String,
    pub reinstall: String
}

impl Flight {
    // pub fn preflight(&self) -> bool {
    //     println!("Running test for {}", self.program);
    //     let mut preflight_command = Command::new(&self.program);
    //     for arg in &self.preflight_args {
    //         preflight_command.arg(&arg);
    //     }
    //     match preflight_command.output() {
    //         Ok(result) => {
    //             let output = String::from_utf8(result.stdout).unwrap();
    //             let errors = String::from_utf8(result.stderr).unwrap();
    //             self.check_preflight_result(output, errors)
    //         }
    //         Err(error) => {
    //             println!("Problem executing program {}: {}", &self.program, error);
    //             false
    //         }
    //     }
    // }

    // fn check_preflight_result(&self, output: String, errors: String) -> bool {
    //     if !errors.is_empty() {
    //         println!("Test failed!");
    //         return false;
    //     }
    //     println!("Test ran without errors!");
    //     if !output.contains(&self.preflight_confirm) {
    //         println!("Test did not contain expected output!");
    //         return false;
    //     }
    //     println!("Test complete!");
    //     true
    // }

    pub fn new(data: HashMap<String, String>) -> Option<Flight> {
        let name = data.get("flight_name")?;
        let flight_exec = data.get("flight_exec")?;
        let preflight_args = data.get("preflight_args")?;
        let stream = data.get("stream")?;
        let flag = data.get("flag")?;
        let package_name = data.get("package_name")?;
        let reinstall = data.get("reinstall")?;
        Some(Flight {
            name: name.clone(),
            flight_exec: flight_exec.clone(),
            preflight_args: preflight_args
                                .split("|")
                                .map(|x| x.to_string())
                                .collect(),
            stream: stream.clone(),
            flag: flag.clone(),
            package_name: package_name.clone(),
            reinstall: reinstall.clone()
        })
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
            // if !flight.preflight() {
            //     return false;
            // }
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

#[derive(Debug)]
pub enum FlightCreateError {
    InvalidParameter,
}
