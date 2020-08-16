use rustdemo::Find;
use structopt::StructOpt;

fn main() {
    let find = Find::from_args();
    let matches = find.find_matches();
    find.list_matches(matches);
}
