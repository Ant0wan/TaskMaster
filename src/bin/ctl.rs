use std::process::exit;
use taskmaster::ctl::cli::parse_args;
use taskmaster::ctl::cli::print_usage;
use taskmaster::ctl::cli::Args;
use taskmaster::ctl::exec::exec;


fn main() {
    let args: Args = parse_args();
    if args.help {
        print_usage();
    }
    exec();

    exit(0)
}
