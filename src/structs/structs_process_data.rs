use std::collections::HashMap;

/// Various structs used to process data from CSV files
/// 
/// This module contains the `Columns` struct, which holds the indices of the relevant columns in the CSV files,
/// the `Review` struct, which represents a review entry, and the `TopGames` struct, which holds information about
/// the top games and their reviews.
/// 
/// Columns struct:
/// 
/// - `game`: usize. Index of the game name column
/// - `language`: usize. Index of the language column
/// - `text`: usize. Index of the review text column
/// - `helpful_votes`: usize. Index of the helpful votes column
/// 
/// Review struct:
/// 
/// - `game`: String. Name of the game
/// - `language`: String. Language of the review
/// - `text`: String. Text of the review
/// - `votes`: u32. Number of helpful votes for the review
/// 
/// TopGames struct:
/// 
/// - `total_review_count`: u32. Total number of reviews for all games
/// - `top_games_languages`: Hashmap<String, TopGamesLanguages>. A HashMap where the key is the language and the value is a `TopGamesLanguages` struct
/// 
/// TopGamesLanguages struct:
/// 
/// - `review_count`: u32. Number of reviews in that language
/// - `top_review`: String. The text of the top review in that language
/// - `top_review_count`: u32. Number of helpful votes for the top review
/// 

pub struct Columns {
    pub game: usize,
    pub language: usize,
    pub text: usize,
    pub helpful_votes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Review {
    pub game: String,
    pub language: String,
    pub text: String,
    pub votes: u32,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct TopGames {
    pub total_review_count: u32,
    pub top_games_languages: HashMap<String, TopGamesLanguages>,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct TopGamesLanguages {
    pub review_count: u32,
    pub top_review: String,
    pub top_review_count: u32,
}

