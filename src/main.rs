use structopt::StructOpt;

use stay_awake::Args;
use std::process;

fn main() {
    let args: Args = Args::from_args();
    if let Err(e) = stay_awake::run(args) {
        println!("Stopping with error: {}", e);
        process::exit(1);
    }
    process::exit(0);
}