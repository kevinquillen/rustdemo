use structopt::StructOpt;
use std::io::{prelude::*, BufReader};
use std::fs::File;

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
    path: std::path::PathBuf
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    let file = File::open(args.path).expect("\n\nCould not read from file!");
    let reader = BufReader::new(file); 

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(&args.pattern) {
            println!("\nFound text:\n\n{}", line.trim());
            break;
        }
    }

    Ok(())
}
