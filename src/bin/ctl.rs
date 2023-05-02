use taskmaster::ctl::cli::{parse_args, print_usage};

fn main() {
    let args = parse_args();
    if args.help {
        print_usage();
    }

    std::process::exit(0)
}
