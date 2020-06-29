use structopt::StructOpt;
use std::io::{prelude::*, BufReader, Result};
use std::fs::File;
use std::path::PathBuf;

#[derive(StructOpt)]
struct CommandOptions {
    #[structopt(
        short = "p", 
        long = "pattern",
        help = "The text you want to look for.",
    )]
    pattern: String,

    #[structopt(
        short = "f",
        long = "filepath",
        help = "The directory to search for the pattern given.",
    )]
    path: PathBuf
}

fn main() -> Result<()> {
    let args = CommandOptions::from_args();
    let file = File::open(args.path).expect("\n\nCould not read from file!");
    let reader = BufReader::new(file); 
    let mut line_number: i32 = 1;
    let mut found = false;

    for line in reader.lines() {
        let line = line.unwrap();
    
        if find_match(line, &args.pattern) {
            found = true;
            break;
        }
        
        line_number = line_number + 1;
    }

    let result = match found {
        false => String::from(format!("Text \"{}\" not found.", args.pattern)),
        true => String::from(format!("Found text \"{}\" on line #{}.", args.pattern, line_number)),
    };

    println!("\n\n{}", result);
    Ok(())
}

fn find_match(content: String, pattern: &str) -> bool {
    return content.to_lowercase().contains(pattern);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_match() {
        let content = String::from("Hello there");
        let pattern = "there";
        assert_eq!(true, find_match(content, pattern));

        let content = String::from("Goodbye!");
        let pattern = "there";
        assert_eq!(false, find_match(content, pattern));

        let content = String::from("ParTiAL");
        let pattern = "part";
        assert_eq!(true, find_match(content, pattern));
    }
}