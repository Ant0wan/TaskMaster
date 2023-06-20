use std::process::exit;
use taskmaster::ctl::cli::parse_args;
use taskmaster::ctl::cli::print_usage;
use taskmaster::ctl::cli::Args;

fn main() {
    let args: Args = parse_args();
    if args.help {
        print_usage();
    }

    exit(0)
}
