use serde::Serialize;
/// Struct to represent the output JSON data
/// This struct is used to serialize the data into JSON format.
/// It contains the following fields:
/// - `padron`: u32. A unique identifier for the data.
/// - `top_games`: `Vec<TopGamesEntry>`. A vector of `TopGamesEntry` structs representing the top games.
/// - `top_languages`: `Vec<TopLanguagesEntry>`. A vector of `TopLanguagesEntry` structs representing the top languages.
///
/// Each `TopGamesEntry` struct contains:
/// - `game`: String. The name of the game.
/// - `review_count`: u32. The total number of reviews for the game.
/// - `languages`: `Vec<LanguageEntry>`. A vector of `LanguageEntry` structs representing the languages for the game.
///
/// Each `LanguageEntry` struct contains:
/// - `language`: String. The name of the language.
/// - `review_count`: u32. The number of reviews in that language.
/// - `top_review`: String. The top review in that language.
/// - `top_review_votes`: u32. The number of votes for the top review.
///
/// Each `TopLanguagesEntry` struct contains:
/// - `language`: String. The name of the language.
/// - `review_count`: u32. The total number of reviews in that language.
/// - `top_reviews`: `Vec<TopReviewEntry>`. A vector of `TopReviewEntry` structs representing the top reviews in that language.
///
/// Each `TopReviewEntry` struct contains:
/// - `review`: String. The text of the review.
/// - `votes`: u32. The number of votes for the review.


#[derive(Serialize, Debug, Default)]
pub struct OutputJson {
    pub padron: u32,
    pub top_games: Vec<TopGamesEntry>,
    pub top_languages: Vec<TopLanguagesEntry>,
}

#[derive(Serialize, Debug, Default)]
pub struct TopGamesEntry {
    pub game: String,
    pub review_count: u32,
    pub languages: Vec<LanguageEntry>,
}

#[derive(Serialize, Debug, Default)]
pub struct LanguageEntry {
    pub language: String,
    pub review_count: u32,
    pub top_review: String,
    pub top_review_votes: u32,
}

#[derive(Serialize, Debug, Default)]
pub struct TopLanguagesEntry {
    pub language: String,
    pub review_count: u32,
    pub top_reviews: Vec<TopReviewEntry>,
}

#[derive(Serialize, Debug, Default)]
pub struct TopReviewEntry {
    pub review: String,
    pub votes: u32,
}