use taskmaster::common::{recognize_file_format, FileFormat};
use taskmaster::d::cli::{parse_args, print_usage, Args};
use taskmaster::d::config::{parse_ini_file, parse_yq_file, Config};

fn main() {
    let args: Args = parse_args();
    println!("{:?}", args); // Debug
    if args.help {
        print_usage();
        std::process::exit(0);
    }
    if args.configuration.is_none() {
        const LOOKAT: [&str; 6] = [
            "../etc/taskmasterd.yaml",
            "../taskmasterd.yaml",
            "./taskmasterd.yaml",
            "./etc/taskmasterd.yaml",
            "/etc/taskmasterdd.yaml",
            "/etc/taskmaster/taskmasterd.conf",
        ];
    }
    if let Some(filename) = args.configuration.as_deref() {
        match recognize_file_format(filename) {
            Some(FileFormat::Yaml) => {
                let config: Config = parse_yq_file(filename).unwrap();
                println!("{:#?}", config); // Debug
            }
            Some(FileFormat::Ini) => {
                let config: Config = parse_ini_file(filename).unwrap();
                println!("{:#?}", config); // Debug
            }
            None => println!("Unrecognized file format"),
        }
    } else {
        println!("No valid configuration file provided.");
    }

    std::process::exit(0);
}
