use rustdemo::Find;
use structopt::StructOpt;

fn main() {
    let find = Find::from_args();
    let matches = find.find_matches();

    if matches.is_empty() {
        println!("No matches found.");
    } else {
        println!("\nFound {} matches: ", matches.len());

        for line in &matches {
            println!(" - Found a match on line #{}.", line)
        }
    }
}
