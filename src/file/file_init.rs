use std::{env, path::PathBuf};
use std::fs::{read_dir, File};
use crate::errors::{Error, ErrorType};


/// Function that reads the console input. Checks if it has more or less arguments.
/// This input is expected to be in the format:
/// ```
/// cargo run <input_path> <num_threads> <output_path>
/// ```
/// # Returns
/// 
/// * `Result<Vec<String>, Error>`. Returns the console input splitted or an Error if there was a problem.

pub fn read_console() -> Result<Vec<String>, Error> {
    let console: Vec<String> = env::args().collect();
    if console.len() < 4 || console.len() > 4 {
        return Err(Error {
            type_error: ErrorType::InputError,
            message: "Invalid arguments".to_string(),
        });
    }

    Ok(console)
}

/// Function that reads the CSV files from the given path.
/// It checks if the path is valid and if it contains CSV files.
/// 
/// # Arguments
/// 
/// * `path`: `&str`. The path to the directory containing the CSV files.
/// 
/// # Returns
/// 
/// * `Result<Vec<File>, Error>`. Returns a vector of CSV files or an Error if there was a problem.

pub fn open_csv(path: &str) -> Result<Vec<File>, Error> {
    println!("Reading files from {}...", path);

    let csv_files = read_dir(path).map_err(|err| Error {
        type_error: ErrorType::InvalidPath,
        message: format!("Failed to read directory. Error: {}", err),
    })?
    .filter_map(|entry| {
        let path = entry.ok()?.path();
        if path.extension()?.to_str()? == "csv" {
            Some(path)
        } else {
            None
        }
    })
    .collect::<Vec<PathBuf>>();

    let files = csv_files.iter().map(|file| {
        let file = File::open(file).map_err(|err| Error {
            type_error: ErrorType::InvalidPath,
            message: format!("Failed to open file. Error: {}", err),
        })?;
        Ok(file)
    }).collect::<Result<Vec<_>, _>>()?;

    Ok(files)
}


