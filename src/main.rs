use std::{io::{self, Write}, env, fs, path::{Path, PathBuf}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (multimc_folder, output_folder) = if args.len() != 3 {
        let exe_name = Path::new(&args[0]).file_name().unwrap().to_str().unwrap();
        println!("Typical Usage: {} <MultiMC folder> <output folder>", exe_name);
        let multimc_folder = prompt_user("Please enter the MultiMC folder: ");
        let output_folder = prompt_user("Please enter the output folder: ");
        (multimc_folder, output_folder)
    } else {
        (args[1].clone(), args[2].clone())
    };

    if !Path::new(&multimc_folder).exists() {
        println!("The MultiMC folder provided does not exist");
        std::process::exit(1);
    }

    if let Err(e) = create_output_folder(&output_folder) {
        println!("Failed to create output folder: {}", e);
        std::process::exit(1);
    }

    let instance_folders = match get_instance_folders(&multimc_folder) {
        Ok(folders) => folders,
        Err(e) => {
            println!("The MultiMC folder provided is not valid");
            std::process::exit(1);
        }
    };

    let total_screenshots = copy_screenshots(instance_folders, &output_folder);

    println!("Total screenshots copied: {}", total_screenshots);
}

fn prompt_user(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn create_output_folder(output_folder: &str) -> Result<(), io::Error> {
    if !Path::new(output_folder).exists() {
        fs::create_dir(output_folder)?;
    }
    Ok(())
}

fn get_instance_folders(multimc_folder: &str) -> Result<Vec<PathBuf>, io::Error> {
    let instance_folder = PathBuf::from(multimc_folder).join("instances");
    let mut folders = Vec::new();
    for entry in fs::read_dir(instance_folder)? {
        let entry = entry?;
        if entry.path().is_dir() {
            folders.push(entry.path());
        }
    }
    Ok(folders)
}

fn copy_screenshots(instance_folders: Vec<PathBuf>, output_folder: &str) -> usize {
    let mut total_screenshots = 0;

    for instance_folder in instance_folders {
        let screenshots_folder = instance_folder.join(".minecraft").join("screenshots");
        if screenshots_folder.exists() {
            for entry in fs::read_dir(screenshots_folder).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap();
                    let dest_path = Path::new(output_folder).join(file_name);
                    if !dest_path.exists() {
                        fs::copy(&path, &dest_path).unwrap();
                        println!("Copied: {:?}", path);
                        total_screenshots += 1;
                    }
                }
            }
        }
    }

    total_screenshots
}