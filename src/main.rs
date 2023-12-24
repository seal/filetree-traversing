use colored::Colorize;

use std::{fs, io};

fn menu(base_path: &str) -> io::Result<()> {
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
                println!("{} {}", file_name.to_string_lossy(), file_metadata.len());
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

fn main() {
    match menu("/") {
        Ok(()) => {
            // Handle success if needed
            println!("Menu executed successfully");
        }
        Err(err) => {
            // Handle error
            eprintln!("Error: {}", err);
        }
    }
}

