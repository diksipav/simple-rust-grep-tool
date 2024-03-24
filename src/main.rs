use std::time::Instant;

use crate::util::read_file_line_by_line_better;
use clap::Parser;

mod util;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Record the start time
    let start_time = Instant::now();

    let args = Cli::parse();

    // read_file_line_by_line(&args)?;
    // read_file_as_string(&args)?;
    read_file_line_by_line_better(&args);

    // let filename = "random_string.txt";
    // let lines = 1000;
    //
    // // Create the random string file
    // create_random_ascii_string_file(filename, lines, 80)?;
    //
    // println!("Random string file '{}' created successfully.", filename);

    let end_time = Instant::now();

    // Calculate the elapsed time
    let elapsed_time = end_time.duration_since(start_time);

    // Print the elapsed time in milliseconds
    println!("Program execution time: {} ms", elapsed_time.as_millis());

    Ok(())
}
