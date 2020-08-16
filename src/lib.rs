use regex::Regex;
use std::io::{prelude::*, BufReader};
use std::fs::File;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
pub struct Find {
    #[structopt(
        short = "t",
        long = "text",
        help = "The text you want to look for.",
    )]
    pub text: String,

    #[structopt(
        short = "f",
        long = "file",
        help = "The file to search for the given text.",
    )]
    pub path: PathBuf
}

impl Find {
    pub fn find_matches(&self) -> Vec<i32> {
        let file = File::open(self.path.to_str().unwrap()).expect("\n\nCould not read from file!");
        let reader = BufReader::new(file);
        let re = Regex::new(format!(r#"(?i){}"#, self.text).as_str()).unwrap();
        let mut matches = Vec::new();
        let mut line_number = 1;

        for line in reader.lines() {
            if re.is_match(line.unwrap().as_str()) {
                matches.push(line_number);
            }

            line_number = line_number + 1;
        }

        matches
    }

    pub fn list_matches(&self, matches: Vec<i32>) {
        if matches.is_empty() {
            println!("No matches found.");
        } else {
            println!("\nFound {} matches: ", matches.len());

            for line in &matches {
                println!(" - Found a match on line #{}.", line)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matches() {
        let fixture = "fixtures/example.txt";

        let find = Find { text: String::from("HELLO THERE!"), path: PathBuf::from(fixture) };
        let matches = find.find_matches();
        assert_eq!(1, matches.len());
        assert_eq!(true, matches.contains(&1));

        let find = Find { text: String::from("nothing to find"), path: PathBuf::from(fixture) };
        let matches = find.find_matches();
        assert_eq!(0, matches.len());
        assert_eq!(true, matches.is_empty());

        let find = Find { text: String::from("Hello there!"), path: PathBuf::from(fixture) };
        let matches = find.find_matches();
        assert_eq!(1, matches.len());
        assert_eq!(true, matches.contains(&1));

        let find = Find { text: String::from("dol"), path: PathBuf::from(fixture) };
        let matches = find.find_matches();
        assert_eq!(2, matches.len());
        assert_eq!(true, matches.contains(&3));
        assert_eq!(true, matches.contains(&7));
    }
}