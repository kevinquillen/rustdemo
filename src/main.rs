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
    let file = File::open(args.path).expect("\n\nCould not read from file!");
    let reader = BufReader::new(file); 
    let mut line_number = 1;
    let mut found = false;

    for line in reader.lines() {
        let line = line.unwrap();
        
        // @todo: accumulate line number and concat for result output instead of just stopping.
        if find_match::find_match(line, &args.pattern) {
            found = true;
            break;
        }
        
        line_number = line_number + 1;
    }

    match found {
        false => println!("Text \"{}\" not found.", args.pattern),
        true => println!("Found text \"{}\" on line #{}.", args.pattern, line_number),
    };
}
