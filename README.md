# Minecraft MultiMC Screenshot Copier

This utility application copies all the screenshots from the instances folders within a MultiMC directory to an output folder.

![image](https://github.com/user-attachments/assets/a4a011de-4199-4cb5-89eb-ec0d6a8f9310)

I found myself doing this frequently after having multiple instances created, and wanted a program to do it for me.

## Usage
1. Download the latest Release and place it wherever you want
2. Run the mc_screenshot_copy.exe and follow the prompts

## Command Line Usage
1. Download the latest Release and place it wherever you want
2. In terminal, browse to the location of the executable you downloaded
3. Execute the program, using your MultiMC folder location and the output folder location you desire:
```sh
.\mc_screenshot_copy.exe <MultiMC folder> <output folder>
```
## Usage notes
- The MultiMC folder is the folder that holds MultiMC.exe (and the instances subdirectory)
- The output folder will be created for you if it doesn't exist

## Dependencies
This project uses the following crate dependencies:

- [`colored`](https://crates.io/crates/colored) for colored terminal output
