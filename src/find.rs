use regex::Regex;
use std::io::{prelude::*, BufReader};
use std::fs::File;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
pub struct FindText {
    #[structopt(
        short = "p",
        long = "pattern",
        help = "The text you want to look for.",
    )]
    pub pattern: String,

    #[structopt(
        short = "f",
        long = "filepath",
        help = "The directory to search for the pattern given.",
    )]
    pub path: PathBuf
}

fn find_string(content: String, pattern: &str) -> bool {
    return Regex::new(format!(r#"(?i){}"#, pattern).as_str()).unwrap().is_match(&content.as_str());
}

pub fn find_matches(pattern: &String, path: &PathBuf) -> Vec<i32> {
    let file = File::open(path).expect("\n\nCould not read from file!");
    let reader = BufReader::new(file);
    let mut matches = Vec::new();
    let mut line_number = 1;

    for line in reader.lines() {
        if find_string(line.unwrap(), pattern) {
            matches.push(line_number);
        }

        line_number = line_number + 1;
    }

    matches
}

pub fn output_matches(matches: Vec<i32>, pattern: &String, path: &PathBuf) {
    match matches.len() > 0 {
        false => println!("Text \"{}\" not found in {:?}.", pattern, path),
        true => {
            println!("\nFound {} matches: ", matches.len());

            for line in &matches {
                println!(" - Found a match on line #{}.", line)
            }
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_string() {
        let content = String::from("Hello there.");
        let pattern = "there";
        assert_eq!(true, find_string(content, pattern));

        let content = String::from("Goodbye!");
        let pattern = "GOOD";
        assert_eq!(true, find_string(content, pattern));

        let content = String::from("I am a ParTiAL mixed case");
        let pattern = "part";
        assert_eq!(true, find_string(content, pattern));

        let content = String::from("good bye");
        let pattern = "hello";
        assert_eq!(false, find_string(content, pattern));

        let content = String::from("hel lo");
        let pattern = "hello";
        assert_eq!(false, find_string(content, pattern));
    }

    #[test]
    fn test_find_matches() {
        let pattern = String::from("Hello there.");
        let path = PathBuf::from("fixtures/example.txt");
        let matches = find_matches(&pattern, &path);
        assert_eq!(1, matches.len());
        assert_eq!(true, matches.contains(&1));

        let pattern = String::from("dol");
        let path = PathBuf::from("fixtures/example.txt");
        let matches = find_matches(&pattern, &path);
        assert_eq!(2, matches.len());
        assert_eq!(true, matches.contains(&3));
        assert_eq!(true, matches.contains(&7));
    }
}