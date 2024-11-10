use std::{io::{self, Read, Write}, env, fs, path::{Path, PathBuf}};
use mc_screenshot_copy::config::Config;
use colored::*;

fn main() {
    setup_terminal();
    print_title();
    
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);

    prompt_ready_to_copy(&config);

    let instance_folders: Vec<PathBuf> = get_instance_folders(&config.multimc_folder);

    let total_screenshots: usize = copy_screenshots(instance_folders, &config.output_folder);

    println!("{} {}", "Total screenshots copied:".magenta(), total_screenshots.to_string().bright_green());

    await_exit_confirmation();
}

/// Sets up the terminal to enable colors on Windows.
fn setup_terminal() {
    if cfg!(target_os = "windows") {
        control::set_virtual_terminal(true).unwrap();
    }
}

/// Prints the title and version of the application.
fn print_title() {
    let title: String = format!(
        "{} {} {} {}", 
        "MultiMC Screenshot Copier".cyan(),
        env!("CARGO_PKG_VERSION").green(),
        "by:".cyan(),
        env!("CARGO_PKG_AUTHORS").green()
    );
    println!("{}", title);
    println!("{}", "=".repeat(title.len()).cyan());
}

/// Prompts the user if they are ready to copy screenshots.
fn prompt_ready_to_copy(config: &Config) {
    loop {
        print!("\nCopy Screenshots from {} to {} (yes/no): ", config.multimc_folder.bright_cyan(), config.output_folder.bright_cyan());
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input: String = String::new();
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

/// Gets the instance folders from the MultiMC folder.
fn get_instance_folders(multimc_folder: &str) -> Vec<PathBuf> {
    let instance_folder: PathBuf = PathBuf::from(multimc_folder).join("instances");
    let mut folders: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = fs::read_dir(instance_folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    folders.push(entry.path());
                }
            }
        }
    }
    folders
}

/// Copies the screenshots from the /.minecraft/screenshots folder within the MultiMC instances to the output folder.
/// Returns the total number of screenshots copied.
fn copy_screenshots(instance_folders: Vec<PathBuf>, output_folder: &str) -> usize {
    let mut total_screenshots: usize = 0;

    for instance_folder in instance_folders {
        let screenshots_folder = instance_folder.join(".minecraft").join("screenshots");
        if screenshots_folder.exists() {
            for entry in fs::read_dir(screenshots_folder).unwrap() {
                let entry: fs::DirEntry = entry.unwrap();
                let path: PathBuf = entry.path();
                if path.is_file() {
                    let file_name: &std::ffi::OsStr = path.file_name().unwrap();
                    let dest_path: PathBuf = Path::new(output_folder).join(file_name);
                    if !dest_path.exists() {
                        fs::copy(&path, &dest_path).unwrap();
                        println!("{} {}", "Copied:".magenta(), path.display().to_string().bright_cyan());
                        total_screenshots += 1;
                    }
                }
            }
        }
    }
    
    total_screenshots
}

/// Prompts the user to press Enter to exit the application.
fn await_exit_confirmation() {
    println!("{}","\nPress Return to exit...".bright_green());
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}