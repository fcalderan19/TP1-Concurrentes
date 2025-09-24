use std::{collections::HashMap, time::Instant};
use std::fs::File;
use csv::Reader;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{errors::{Error, ErrorType}, structs::{structs_process_data::Review, structs_json::{TopReviewEntry, TopLanguagesEntry}}};
use crate::structs::structs_process_data::{Columns, TopGames};


/// Function that processes the CSV files and gathers the data.
/// 
/// # Arguments
/// 
/// * `files`: `Vec<File>`. A vector of CSV files to process.
/// 
/// # Returns
/// 
/// * `Result<(HashMap<String, TopGames>, HashMap<String, (u32, TopLanguagesEntry)>), Error>`. Returns a tuple containing two HashMaps:
/// - The first HashMap contains the top games and their review counts.
/// - The second HashMap contains the top languages and their review counts.
/// 

pub fn gather_data(files: Vec<File>) -> Result<(HashMap<String, TopGames>, HashMap<String, (u32, TopLanguagesEntry)>), Error> {
    println!("Processing files...");
    let start = Instant::now();

    let (top_games, top_languages) = files
        .par_iter()
        .map(|file| {
            let mut reader = Reader::from_reader(file);
            let headers = reader.headers().map_err(|e| Error {
                type_error: ErrorType::IOError,
                message: e.to_string(),
            })?.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            let columns = columns_index(headers)?;

            let mut games_acc: HashMap<String, TopGames> = HashMap::new();
            let mut langs_acc: HashMap<String, (u32, TopLanguagesEntry)> = HashMap::new();

            for result in reader.records() {
                let record = result.map_err(|e| Error {
                    type_error: ErrorType::IOError,
                    message: e.to_string(),
                })?;
                let fields: Vec<String> = record.iter().map(|s| s.to_string()).collect();
                let votes = fields[columns.helpful_votes].parse::<u32>().unwrap_or_default();

                let review = Review {
                    game: fields[columns.game].clone(),
                    language: fields[columns.language].clone(),
                    text: fields[columns.text].clone(),
                    votes,
                };

                let game_entry = games_acc.entry(review.game.clone()).or_default();
                game_entry.total_review_count += 1;
                
                let lang_in_game = game_entry.top_games_languages
                    .entry(review.language.clone())
                    .or_default();
                
                lang_in_game.review_count += 1;
                if votes > lang_in_game.top_review_count {
                    lang_in_game.top_review = review.text.clone();
                    lang_in_game.top_review_count = votes;
                }

                let (count, lang_entry) = langs_acc
                    .entry(review.language.clone())
                    .or_default();
                
                *count += 1;
                if votes > 0 {
                    lang_entry.top_reviews.push(TopReviewEntry {
                        review: review.text.clone(),
                        votes,
                    });
                }
            }
            println!("Total gathering time: {:?}", start.elapsed());
            Ok((games_acc, langs_acc))
        })
        .reduce(
            || Ok((HashMap::new(), HashMap::new())),
            |a, b| match (a, b) {
                (Ok((a_games, a_langs)), Ok((b_games, b_langs))) => {
                    let start = Instant::now();
                    println!("Reducing data...");
                    let merged_games = merge_game_maps(a_games, b_games);
                    let merged_langs = merge_lang_maps(a_langs, b_langs);
                    println!("Total reduction time: {:?}", start.elapsed());
                    Ok((merged_games, merged_langs))
                }
                (Err(e), _) | (_, Err(e)) => Err(e),
            },

        )?;

    Ok((top_games, top_languages))
}

/// Function that merges two HashMaps of TopGames.
/// 
/// # Arguments
/// 
/// * `a`: `HashMap<String, TopGames>`. The first HashMap to merge.
/// * `b`: `HashMap<String, TopGames>`. The second HashMap to merge.
/// 
/// # Returns
/// 
/// * `HashMap<String, TopGames>`. The merged HashMap.
///
/// This function takes two HashMaps of TopGames and merges them into one.
/// It iterates over the second HashMap and adds the review counts to the first HashMap.
/// If a game already exists in the first HashMap, it updates the review counts.
/// If a game does not exist in the first HashMap, it adds it.
/// It also updates the top review and its count for each language.
///

fn merge_game_maps(
    mut a: HashMap<String, TopGames>,
    b: HashMap<String, TopGames>,
) -> HashMap<String, TopGames> {
    for (game_name, stats) in b {
        let entry = a.entry(game_name).or_default();
        entry.total_review_count += stats.total_review_count;
        
        for (lang, lang_stats) in stats.top_games_languages {
            let target = entry.top_games_languages.entry(lang).or_default();
            target.review_count += lang_stats.review_count;
            if lang_stats.top_review_count > target.top_review_count {
                target.top_review = lang_stats.top_review;
                target.top_review_count = lang_stats.top_review_count;
            }
        }
    }
    a
}

/// Function that merges two HashMaps of TopLanguages.
/// 
/// # Arguments
/// 
/// * `a`: `HashMap<String, (u32, TopLanguagesEntry)>`. The first HashMap to merge.
/// * `b`: `HashMap<String, (u32, TopLanguagesEntry)>`. The second HashMap to merge.
/// 
/// # Returns
/// 
/// * `HashMap<String, (u32, TopLanguagesEntry)>`. The merged HashMap.
/// 
/// This function takes two HashMaps of TopLanguages and merges them into one.
/// It iterates over the second HashMap and adds the review counts to the first HashMap.
/// If a language already exists in the first HashMap, it updates the review counts.
/// If a language does not exist in the first HashMap, it adds it.
/// It also updates the top reviews for each language.
/// 

fn merge_lang_maps(
    mut a: HashMap<String, (u32, TopLanguagesEntry)>,
    b: HashMap<String, (u32, TopLanguagesEntry)>,
) -> HashMap<String, (u32, TopLanguagesEntry)> {
    for (lang, (count, entry)) in b {
        let (total_count, target_entry) = a.entry(lang).or_default();
        *total_count += count;
        target_entry.top_reviews.extend(entry.top_reviews);
    }
    a
}


/// A function that takes a vector of headers and returns a Columns struct with the indices of the relevant columns.
/// This is used to determine the indices of the columns in the CSV files in case the columns are out of order.
/// 
/// # Arguments
/// 
/// * `headers`: `Vec<String>`. A vector of strings representing the headers of the CSV file.
/// 
/// # Returns
/// 
/// * `Result<Columns, Error>`. Returns a Columns struct with the indices of the relevant columns or an Error if there was a problem.

fn columns_index(headers: Vec<String>) -> Result<Columns, Error> {
    if headers.len() < 4 {
        return Err(Error {
            type_error: ErrorType::ColumnsError,
            message: "File does not contain enough columns".to_string(),
        });
    }

    let app_name_index = headers.iter().position(|s| s == "app_name").ok_or_else(|| Error {
        type_error: ErrorType::ColumnsError,
        message: "Column 'app_name' not found".to_string(),
    })?;

    let language_index = headers.iter().position(|s| s == "language").ok_or_else(|| Error {
        type_error: ErrorType::ColumnsError,
        message: "Column 'language' not found".to_string(),
    })?;

    let review_index = headers.iter().position(|s| s == "review").ok_or_else(|| Error {
        type_error: ErrorType::ColumnsError,
        message: "Column 'review' not found".to_string(),
    })?;

    let votes_helpful_index = headers.iter().position(|s| s == "votes_helpful").ok_or_else(|| Error {
        type_error: ErrorType::ColumnsError,
        message: "Column 'votes_helpful' not found".to_string(),
    })?;

    let column = Columns {
        game: app_name_index,
        language: language_index,
        text: review_index,
        helpful_votes: votes_helpful_index,
    };

    Ok(column)
}
