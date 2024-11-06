use std::{env, fs, path::{Path, PathBuf}};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <MultiMC folder> <output folder>", args[0]);
        std::process::exit(1);
    }

    let multimc_folder = &args[1];
    let output_folder = &args[2];

    if !Path::new(multimc_folder).exists() {
        println!("The MultiMC folder does not exist");
        std::process::exit(1);
    }

    create_output_folder(output_folder);

    let instance_folders = get_instance_folders(multimc_folder);

    let total_screenshots = copy_screenshots(instance_folders, output_folder);

    println!("Total screenshots copied: {}", total_screenshots);
}

fn create_output_folder(output_folder: &str) {
    if !Path::new(output_folder).exists() {
        fs::create_dir(output_folder).unwrap();
    }
}

fn get_instance_folders(multimc_folder: &str) -> Vec<PathBuf> {
    let instance_folder = PathBuf::from(multimc_folder).join("instances");
    fs::read_dir(instance_folder)
        .unwrap()
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect()
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