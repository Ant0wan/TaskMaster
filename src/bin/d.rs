use taskmaster::d::cli::{parse_args, print_usage, Args};
use taskmaster::d::config_parser::parse_ini_file;
use taskmaster::d::config_parser::IniConfig;

fn main() {
    let args: Args = parse_args();
    if args.help {
        print_usage()
    }
    if let Some(filename) = args.configuration.as_deref() {
        let config: IniConfig = parse_ini_file(filename).unwrap();
        println!("{:#?}", config);
    } else {
        println!("No valide configuration file provided.");
    }

    std::process::exit(0)
}
