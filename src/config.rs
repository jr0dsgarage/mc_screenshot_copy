/// Struct to hold the configuration for the application.
use std::{fs, io::{self, Write}, path::Path};
use colored::*;
pub struct Config {
    pub multimc_folder: String,
    pub output_folder: String,
}

impl Config {
    /// Creates a new Config instance from command-line arguments or prompts the user for input.
    pub fn new(args: &[String]) -> Config {
        let multimc_folder: String;
        let output_folder: String;

        if args.len() != 3 {
            println!("Typical command prompt Usage: {} {}",
                Path::new(&args[0]).file_name().unwrap().to_str().unwrap().bright_green(),
                "<MultiMC folder path> <output folder path>".bright_green());
            println!("{}","Not enough arguments provided, prompting for folder paths...".bright_red());
            multimc_folder = Self::folder_prompt("Please enter the MultiMC folder path: ");
            output_folder = Self::folder_prompt("Please enter the desired output folder path: ");
        } else {
            multimc_folder = args[1].clone();
            output_folder = args[2].clone();
        }

        let config: Config = Config { multimc_folder, output_folder };
        config.validate()
    }

        /// Validates the configuration and re-prompts the user for valid inputs if necessary.
        fn validate(mut self) -> Config {
            loop {
                if let Err(e) = self.validate_multimc_folder() {
                    println!("Error: {}", e.bright_red());
                    self.multimc_folder = Self::folder_prompt("Please enter a valid MultiMC folder path: ");
                    continue;
                }
                break;
            }
    
            loop {
                if let Err(e) = self.validate_output_folder() {
                    println!("Error: {}", e.bright_red());
                    self.output_folder = Self::folder_prompt("Please enter a valid output folder path: ");
                    continue;
                }
                break;
            }
    
            self
        }
    

    /// Validates the MultiMC folder path.
    fn validate_multimc_folder(&self) -> Result<(), String> {
        if self.multimc_folder.is_empty() {
            Err("No input given for the MultiMC folder".to_string())
        } else if !Path::new(&self.multimc_folder).exists() {
            Err("The MultiMC folder provided does not exist".to_string())
        } else {
            let instance_folder = Path::new(&self.multimc_folder).join("instances");
            if !instance_folder.exists() {
                Err("The MultiMC folder does not contain an 'instances' folder".to_string())
            } else if fs::read_dir(instance_folder).map_err(|e| e.to_string())?.next().is_none() {
                Err("The 'instances' folder does not contain any instance folders".to_string())
            } else {
                Ok(())
            }
        }
    }

    /// Validates the output folder path and creates the folder if it doesn't exist.
    pub fn validate_output_folder(&self) -> Result<(), String> {
        if self.output_folder.is_empty() {
            Err("No value given for the output folder".to_string())
        } else if !Path::new(&self.output_folder).exists() {
            if let Err(e) = fs::create_dir_all(&self.output_folder) {
                Err(format!("Failed to create output folder: {}", e.to_string()))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    /// Prompts the user for a folder path.
    fn folder_prompt(prompt: &str) -> String {
        print!("{}", prompt.bright_green());
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }
    

}
