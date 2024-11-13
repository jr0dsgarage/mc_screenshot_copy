use std::{env, path::PathBuf};
use mc_screenshot_copy::*;
use mc_screenshot_copy::config::Config;
use colored::*;

fn main() {
    setup_terminal();
    print_title();
    
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);

    prompt_ready_to_copy(&config);
    let instance_folders: Vec<PathBuf> = get_instance_folders(&config.multimc_folder);

    let (total_screenshots_copied, total_screenshots_not_copied) = copy_screenshots(instance_folders, &config.output_folder);

    if total_screenshots_not_copied > 0 {
        println!("{} {}", "Total screenshots not copied because they already exist in output folder:".magenta(), total_screenshots_not_copied.to_string().bright_red());
    }
    println!("{} {}", "Total screenshots copied:".magenta(), total_screenshots_copied.to_string().bright_green());

    await_exit_confirmation();
}