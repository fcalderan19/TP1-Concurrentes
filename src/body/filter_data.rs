use std::collections::HashMap;

use crate::structs::structs_process_data::TopGames;
use crate::structs::structs_json::{LanguageEntry, TopGamesEntry, TopLanguagesEntry};
use crate::errors::Error;


/// Function that filters the top games and their languages.
/// 
/// This function takes the result of the parallel processing of the games, to sort them and limit the amount of
/// games shown in order to the assignment requirements.
/// 
/// # Arguments
/// 
/// * `data`: HashMap<String, TopGames>. A HashMap where the key is the game name and the value is a `TopGames` struct
/// 
/// # Returns
/// 
/// * `Result<Vec<TopGamesEntry>, Error>`. Returns a vector of `TopGamesEntry` structs representing the top games.
///   If there was a problem, it returns an Error.

pub fn filter_top_games(data: HashMap<String, TopGames>) -> Result<Vec<TopGamesEntry>,Error> {
    let mut top_games_entries: Vec<TopGamesEntry> = data.into_iter().map(|(game_name, top_game)| {
        let languages = top_game.top_games_languages.into_iter().map(|(lang, lang_data)| {
            LanguageEntry {
                language: lang,
                review_count: lang_data.review_count,
                top_review: lang_data.top_review,
                top_review_votes: lang_data.top_review_count,
            }
        }).collect();

        TopGamesEntry {
            game: game_name,
            review_count: top_game.total_review_count,
            languages,
        }
    }).collect();

    top_games_entries.sort_by(|a, b| b.review_count.cmp(&a.review_count));

    let top_3_games = top_games_entries.into_iter().map(|game| {
        let mut game = game;
        game.languages.sort_by(|a, b| b.review_count.cmp(&a.review_count));
        game.languages = game.languages.into_iter().take(3).collect();
        game
    }).take(3).collect();

    Ok(top_3_games)
}



/// Function that filters the top languages.
/// 
/// This function takes the result of the parallel processing of the languages, to sort them and limit the amount of
/// languages shown in order to the assignment requirements.
/// 
/// # Arguments
/// 
/// * `languages`: HashMap<String, (u32, TopLanguagesEntry)>. A HashMap where the key is the language and the value is a tuple
/// 
///  # Returns
/// 
/// * `Result<Vec<TopLanguagesEntry>, Error>`. Returns a vector of `TopLanguagesEntry` structs representing the top languages.
///   If there was a problem, it returns an Error.

pub fn filter_top_languages(languages: HashMap<String, (u32, TopLanguagesEntry)>) -> Result<Vec<TopLanguagesEntry>, Error> {
    let mut top_languages: Vec<TopLanguagesEntry> = languages.into_iter().map(|
        (language, (review_count, mut top_reviews))| {
            top_reviews.top_reviews.sort_by(|a, b| b.votes.cmp(&a.votes));
            let top_reviews = top_reviews.top_reviews.into_iter().take(10).collect();

            TopLanguagesEntry {
                language,
                review_count,
                top_reviews,
            }
        }
    ).collect();

    top_languages.sort_by(|a, b| b.review_count.cmp(&a.review_count));
    
    Ok(top_languages.into_iter().take(3).collect())
}