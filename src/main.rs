mod errors;
mod file;
mod body;
mod structs;

use std::time::Instant;

use body::filter_data::{filter_top_games, filter_top_languages};
use rayon::ThreadPoolBuilder;

use crate::errors::Error;
use crate::file::{file_init::{read_console, open_csv}, file_json::output_json};
use crate::body::process_data::gather_data;

const PADRON: u32 = 85927; // Mi actual padron es 110873 pero puse este porque es el del expected output.

fn main() {
    let start = Instant::now();
    
    if let Err(e) = run_console(){
        println!("{}", e);
    }
    
    println!("");
    println!("Total processing time: {:?}", start.elapsed());
}


/// Function processes the console input. This input is expected to be in the format:
/// 
/// ```
/// cargo run <input_path> <num_threads> <output_path>
/// ```
/// 
/// # Arguments
/// 
/// * `input_path` - Path to the input CSV files
/// * `num_threads` - Number of threads to use
/// * `output_path` - Path to the output JSON file

fn run_console() -> Result<(), Error> {
    println!("Reading console input...");

    let console_input = read_console()?;
    let input_path = &console_input[1];
    let num_threads = &console_input[2];
    let output_path = &console_input[3];

    let threads = num_threads.parse::<usize>().unwrap_or_default();

    ThreadPoolBuilder::new()
    .num_threads(threads)
    .build_global()
    .unwrap();

    let data = gather_data(open_csv(input_path)?)?;

    output_json(output_path, filter_top_games(data.0)?, filter_top_languages(data.1)?)?;

    Ok(())
}
