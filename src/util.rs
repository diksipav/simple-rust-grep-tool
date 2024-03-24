use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

use crate::Cli;

pub fn read_file_as_string(args: &Cli) -> Result<()> {
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read the file {}", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn read_file_line_by_line(args: &Cli) -> Result<()> {
    let f = File::open(&args.path)
        .with_context(|| format!("Could not read the file {}", args.path.display()))?;

    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line?;

        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn read_file_line_by_line_better(args: &Cli) -> Result<()> {
    let f = File::open(&args.path)
        .with_context(|| format!("Could not read the file {}", args.path.display()))?;

    let mut reader = BufReader::with_capacity(3000, f);
    let mut buffer = String::new();

    loop {
        let len = reader.read_line(&mut buffer)?;

        if len == 0 {
            break;
        }

        if buffer.contains(&args.pattern) {
            println!("{}", buffer.trim());
        }
        buffer.clear();
    }

    Ok(())
}

// fn generate_random_ascii_string(lines: usize, characters_per_line: usize) -> String {
//     let mut rng = thread_rng();
//     let mut random_string = String::new();
//
//     // Range of Unicode code points for ASCII characters
//     let ascii_range = ('\u{0020}'..='\u{007E}');
//
//     // Generate random characters
//     for _ in 0..lines {
//         for _ in 0..characters_per_line {
//             let ascii_char: char = rng.gen_range(ascii_range.clone());
//             random_string.push(ascii_char);
//         }
//         random_string.push('\n'); // Add a newline after each line
//     }
//
//     random_string
// }
//
// pub fn create_random_ascii_string_file(
//     filename: &str,
//     lines: usize,
//     characters_per_line: usize,
// ) -> io::Result<()> {
//     let mut file = File::create(filename)?;
//     let random_string = generate_random_ascii_string(lines, characters_per_line);
//     file.write_all(random_string.as_bytes())?;
//     Ok(())
// }
