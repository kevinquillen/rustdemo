use structopt::StructOpt;

mod find;

fn main() {
    let command_args = find::FindText::from_args();
    let hits = find::find_matches(&command_args.pattern, &command_args.path);

    match hits.len() > 0 {
        false => println!("Text \"{}\" not found in {:?}.", command_args.pattern, command_args.path),
        true => {
            println!("\nFound {} matches: ", hits.len());

            for line in &hits {
                println!(" - Found a match on line #{}.", line)
            }
        },
    };
}
