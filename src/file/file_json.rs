use std::fs::File;
use std::path::PathBuf;
use crate::errors::{Error, ErrorType};
use crate::structs::structs_json::{OutputJson, TopGamesEntry, TopLanguagesEntry};
use crate::PADRON;

/// Function that redirects the data to a JSON file.
/// 
/// # Arguments
/// 
/// * `output_path`: &str. Path to the output JSON file
/// * `top_3_games`: `Vec<TopGamesEntry>`. Vector of the top 3 games
/// * `top_3_languages`: `Vec<TopLanguageEntry>`. Vector of the top 3 languages
/// 
/// # Returns
/// 
/// * `Result<(), Error>`. Returns Ok if successful, or an Error if there was a problem

pub fn output_json(output_path: &str, top_3_games: Vec<TopGamesEntry>, top_3_languages: Vec<TopLanguagesEntry>) -> Result<(), Error> {
    println!("Redirecting data to output file...");

    let path = PathBuf::from(output_path);

    let file = File::create(&path).map_err(|err| Error {
        type_error: ErrorType::IOError,
        message: format!("Failed to create file. Error: {}", err),
    })?;

    let data = OutputJson { 
        padron: PADRON, 
        top_games: top_3_games, 
        top_languages: top_3_languages,
    };

    serde_json::to_writer_pretty(file, &data).unwrap();

    Ok(())
}