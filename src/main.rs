use structopt::StructOpt;

mod find;
use find::FindText;

fn main() {
    let op = FindText::from_args();
    let matches = op.find_matches();
    op.output_matches(matches);
}
