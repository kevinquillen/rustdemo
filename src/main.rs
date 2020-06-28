use structopt::StructOpt;
use std::io::{prelude::*, BufReader, Result};
use std::fs::File;
use std::path::PathBuf;

#[derive(StructOpt)]
struct Cli {
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
    let args = Cli::from_args();
    let file = File::open(args.path).expect("\n\nCould not read from file!");
    let reader = BufReader::new(file); 
    let mut line_number: i32 = 1;
    let mut found = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.contains(&args.pattern) {
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
