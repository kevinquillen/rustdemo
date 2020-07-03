use structopt::StructOpt;
use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::path::PathBuf;

mod find_match;

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

fn main() {
    let args = CommandOptions::from_args();
    let file = File::open(&args.path).expect("\n\nCould not read from file!");
    let reader = BufReader::new(file);
    let mut hits = Vec::new();
    let mut line_number = 1;

    for line in reader.lines() {
        if find_match::find_match(line.unwrap(), &args.pattern) {
            hits.push(line_number);
        }
        
        line_number = line_number + 1;
    }

    match hits.len() > 0 {
        false => println!("Text \"{}\" not found in {:?}.", args.pattern, args.path),
        true => {
            println!("\nFound {} matches: ", hits.len());

            for line in &hits {
                println!(" - Found a match on line #{}.", line)
            }
        },
    };
}
