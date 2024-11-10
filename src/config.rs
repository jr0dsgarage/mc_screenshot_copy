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

        // If the user didn't provide enough arguments, do nothing and fill the variables with empty strings.
        // The config.validate() function call will re-prompt the user for input.
        if args.len() != 3 {
            println!("Typical command prompt Usage: {} {}",
                Path::new(&args[0]).file_name().unwrap().to_str().unwrap().bright_green(),
                "<MultiMC folder path> <output folder path>".bright_green());
            println!("{}","Not enough arguments provided, prompting for folder paths...".bright_red());
            multimc_folder = String::new();
            output_folder = String::new();
        } else {
            multimc_folder = args[1].clone();
            output_folder = args[2].clone();
        }

        let mut config: Config = Config { multimc_folder, output_folder };
        config.validate_and_prompt();
        config
    }

    /// Validates the configuration and re-prompts the user for valid inputs if necessary.
    fn validate_and_prompt(&mut self) {
        self.multimc_folder = Self::validate_and_prompt_folder(
            &self.multimc_folder,
            "Please enter the MultiMC folder path: ",
            |path: &str| Self::validate_multimc_folder(path),
        );

        self.output_folder = Self::validate_and_prompt_folder(
            &self.output_folder,
            "Please enter the desired output folder path: ",
            |path: &str| Self::validate_output_folder(path),
        );

        self.prompt_ready_to_copy();
    }

    /// Validates a folder path and prompts the user for valid input if necessary.
    fn validate_and_prompt_folder<F>(initial_path: &str, prompt: &str, validate: F) -> String
    where
        F: Fn(&str) -> Result<(), String>,
    {
        let mut path = initial_path.to_string();
        loop {
            match validate(&path) {
                Ok(_) => break,
                Err(e) => {
                    println!("Error: {}", e.bright_red());
                    path = Self::folder_prompt(prompt);
                }
            }
        }
        path
    }
    
    /// Validates the MultiMC folder path.
    fn validate_multimc_folder(path: &str) -> Result<(), String> {
        if path.is_empty() {
            Err("No input given for the MultiMC folder".to_string())
        } else if !Path::new(path).exists() {
            Err("The MultiMC folder provided does not exist".to_string())
        } else {
            let instance_folder = Path::new(path).join("instances");
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
    fn validate_output_folder(path: &str) -> Result<(), String> {
        if path.is_empty() {
            Err("No value given for the output folder".to_string())
        } else if !Path::new(path).exists() {
            if let Err(e) = fs::create_dir_all(path) {
                Err(format!("Failed to create output folder: {}", e.to_string()))
            } else {
                println!("Created output folder: {}", path.bright_green());
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

    /// Prompts the user if they are ready to copy screenshots.
    fn prompt_ready_to_copy(&self) {
        loop {
            print!("\nCopy Screenshots from {} to {} (yes/no): ", self.multimc_folder.bright_cyan(), self.output_folder.bright_cyan());
            io::stdout().flush().expect("Failed to flush stdout");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().to_lowercase().as_str() {
                "yes" | "y" => break,
                "no" | "n" => {
                    println!("Operation cancelled!");
                    break;
                }
                _ => println!("Please enter 'yes' or 'no'."),
            }
        }
    }
    
}
