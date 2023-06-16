use taskmaster::d::cli::{parse_args, print_usage, Args};
//use taskmaster::d::ini::IniConfig;
use taskmaster::d::config::{parse_ini_file, parse_yq_file, Config};

fn main() {
    let args: Args = parse_args();
    if args.help {
        print_usage()
    }
    if let Some(filename) = args.configuration.as_deref() {
        //let configy: Config = parse_yq_file(filename).unwrap();
        //println!("{:#?}", configy);
        let config: Config = parse_ini_file(filename).unwrap();
        println!("{:#?}", config);
    } else {
        println!("No valide configuration file provided.");
    }

    std::process::exit(0)
}
