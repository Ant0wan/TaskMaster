use taskmaster::common::{recognize_file_format, FileFormat};
use taskmaster::d::cli::{parse_args, print_usage, Args};
use taskmaster::d::config::{parse_ini_file, parse_yq_file, Config};

fn main() {
    let args: Args = parse_args();
    if args.help {
        print_usage()
    }
    if let Some(filename) = args.configuration.as_deref() {
        match recognize_file_format(filename) {
            Some(FileFormat::Yaml) => {
                let config: Config = parse_yq_file(filename).unwrap();
                println!("{:#?}", config);
            }
            Some(FileFormat::Ini) => {
                let config: Config = parse_ini_file(filename).unwrap();
                println!("{:#?}", config);
            }
            None => println!("Unrecognized file format"),
        }
    } else {
        println!("No valid configuration file provided.");
    }

    std::process::exit(0);
}
