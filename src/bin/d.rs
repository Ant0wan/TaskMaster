use std::fs;
use taskmaster::common::{recognize_file_format, FileFormat};
use taskmaster::d::cli::{parse_args, print_usage, Args};
use taskmaster::d::config::{parse_ini_file, parse_yq_file, Config};
use taskmaster::d::exec::exec;

fn main() {
    let mut args: Args = parse_args();
    println!("{:?}", args); // Debug
    if args.help {
        print_usage();
        std::process::exit(0);
    }

    if args.configuration.is_none() {
        const LOOKAT: [&str; 24] = [
            "../etc/supervisord.conf",          // Relative to the executable
            "../supervisord.conf",              // Relative to the executable
            "./supervisord.conf",               // Current working directory
            "./etc/supervisord.conf",           // Current working directory
            "/etc/supervisord.conf",            // Absolute path
            "/etc/supervisor/supervisord.conf", // Supervisor 3.3.0 and above
            "../etc/taskmasterd.yaml",
            "../taskmasterd.yaml",
            "./taskmasterd.yaml",
            "./etc/taskmasterd.yaml",
            "/etc/taskmasterd.yaml",
            "/etc/taskmaster/taskmasterd.yaml",
            "../etc/taskmasterd.json",
            "../taskmasterd.json",
            "./taskmasterd.json",
            "./etc/taskmasterd.json",
            "/etc/taskmasterd.json",
            "/etc/taskmaster/taskmasterd.json",
            "../etc/taskmasterd.conf",
            "../taskmasterd.conf",
            "./taskmasterd.conf",
            "./etc/taskmasterd.conf",
            "/etc/taskmasterd.conf",
            "/etc/taskmaster/taskmasterd.conf",
        ];

        for file_path in LOOKAT.iter() {
            if let Ok(_content) = fs::read_to_string(file_path) {
                println!("configuration file: {:?}", file_path); // Debug
                args.configuration = Some(file_path.to_string());
                break;
            }
        }
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

    // can defer Args
    exec()
}
