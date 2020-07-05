use structopt::StructOpt;

mod find;

fn main() {
    let command_args = find::FindText::from_args();
    let matches = find::find_matches(&command_args.pattern, &command_args.path);
    find::output_matches(matches, &command_args.pattern, &command_args.path);
}
