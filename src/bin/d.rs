use taskmaster::d::{cli::parse_args, cli::print_usage, config_parser::parse_ini_file};

fn main() {
    let args = parse_args();
    if args.help {
        print_usage()
    }
    if let Some(filename) = args.configuration.as_deref() {
        let config = parse_ini_file(filename).unwrap();
        println!("{:#?}", config);
    } else {
        println!("No valide configuration file provided.");
    }

    std::process::exit(0)
}
