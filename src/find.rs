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
    pub text: String,

    #[structopt(
        short = "f",
        long = "filepath",
        help = "The directory to search for the pattern given.",
    )]
    pub path: PathBuf
}

impl FindText {
    fn find_string(&self, content: String) -> bool {
        Regex::new(format!(r#"(?i){}"#, self.text).as_str()).unwrap().is_match(content.as_str())
    }

    pub fn find_matches(&self) -> Vec<i32> {
        let file = File::open(self.path.to_str().unwrap()).expect("\n\nCould not read from file!");
        let reader = BufReader::new(file);
        let mut matches = Vec::new();
        let mut line_number = 1;

        for line in reader.lines() {
            if self.find_string(line.unwrap()) {
                matches.push(line_number);
            }

            line_number = line_number + 1;
        }

        matches
    }

    pub fn output_matches(&self, matches: Vec<i32>) {
        match matches.len() > 0 {
            false => println!("No text was found."),
            true => {
                println!("\nFound {} matches: ", matches.len());

                for line in &matches {
                    println!(" - Found a match on line #{}.", line)
                }
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_string() {
        let fixture = "fixtures/example.txt";

        let op = FindText { text: String::from("there"), path: PathBuf::from(fixture) };
        assert_eq!(true, op.find_string(String::from("Hello there")));

        let op = FindText { text: String::from("GOOD"), path: PathBuf::from(fixture) };
        assert_eq!(true, op.find_string(String::from("Goodbye!")));

        let op = FindText { text: String::from("part"), path: PathBuf::from(fixture) };
        assert_eq!(true, op.find_string(String::from("I am a ParTiAL mixed case")));

        let op = FindText { text: String::from("good bye"), path: PathBuf::from(fixture) };
        assert_eq!(false, op.find_string(String::from("hello")));

        let op = FindText { text: String::from("hel lo"), path: PathBuf::from(fixture) };
        assert_eq!(false, op.find_string(String::from("hello")));
    }

    #[test]
    fn test_find_matches() {
        let fixture = "fixtures/example.txt";

        let op = FindText { text: String::from("Hello there!"), path: PathBuf::from(fixture) };
        let matches = op.find_matches();
        assert_eq!(1, matches.len());
        assert_eq!(true, matches.contains(&1));

        let op = FindText { text: String::from("dol"), path: PathBuf::from(fixture) };
        let matches = op.find_matches();
        assert_eq!(2, matches.len());
        assert_eq!(true, matches.contains(&3));
        assert_eq!(true, matches.contains(&7));
    }
}