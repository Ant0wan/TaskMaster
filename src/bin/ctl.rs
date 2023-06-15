use taskmaster::ctl::cli::{parse_args, print_usage, Args};

fn main() {
    let args: Args = parse_args();
    if args.help {
        print_usage();
    }

    std::process::exit(0)
}
