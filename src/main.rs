//! A module for navigating and displaying file information in a directory.
//!
//! This module provides a `menu` function that allows the user to navigate through directories
//! and view information about files, such as file size and permissions.

use colored::Colorize;
use std::{fs, io, os::unix::fs::PermissionsExt};
/// Displays a menu for navigating and displaying file information in a directory.
///
/// The `menu` function reads the contents of the directory specified by `base_path` and displays
/// the file names, sizes, and permissions. It also allows the user to navigate to subdirectories
/// by entering the file name.
///
/// # Arguments
///
/// * `base_path` - The path to the directory to navigate and display file information for.
///
/// # Returns
///
/// Returns `Ok(())` if the menu executed successfully, or an `Err` if an I/O error occurred.
///
/// # Examples
///
/// ```
/// use filetree::menu;
///
/// match menu("/") {
///     Ok(()) => {
///         println!("Menu executed successfully");
///     }
///     Err(err) => {
///         eprintln!("Error: {}", err);
///     }
/// }
/// ```
pub fn menu(base_path: &str) -> io::Result<()> {
    // random comment for v0.1.6
    let mut exit = false;
    while !exit {
        for entry in fs::read_dir(base_path)? {
            let entry = entry?;
            let file_path = entry.path();
            let file_metadata = fs::metadata(&file_path)?;

            let file_name = entry.file_name();
            if !file_metadata.is_file() {
                println!("{} ", file_name.to_string_lossy().blue(),);
            } else {
                println!(
                    "{} | Bytes: {} | Perms: {:?} ",
                    file_name.to_string_lossy(),
                    file_metadata.len(),
                    file_metadata.permissions().mode()
                );
            }
        }

        println!("Enter file name to navigate to");
        println!("Enter 'exit' to quit or press Enter to continue:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim(); // Remove leading and trailing whitespaces

        if input == "exit" {
            exit = true;
        } else {
            menu(&(base_path.to_owned() + input + "/"))?;
        }
    }

    Ok(())
}
/// The main entry point of the program.
///
/// This function calls the `menu` function with the root directory ("/") as the starting path.
/// It prints a success message if the menu executed successfully, or an error message if an error occurred.
fn main() {
    match menu("/") {
        Ok(()) => {
            println!("Menu executed successfully");
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
