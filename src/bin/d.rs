use std::fs;
use std::process::exit;
use taskmaster::common::recognize_file_format;
use taskmaster::common::FileFormat;
use taskmaster::config::parse_ini_file;
use taskmaster::config::parse_yq_file;
use taskmaster::config::Config;
use taskmaster::d::cli::parse_args;
use taskmaster::d::cli::print_usage;
use taskmaster::d::cli::Args;
use taskmaster::d::exec::exec;

fn main() {
    let mut args: Args = parse_args();
    println!("{:?}", args); // Debug
    if args.help {
        print_usage();
        exit(0);
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
            None => {
                println!("Error: could not find config file totol");
                forhelp();
            }
        }
    } else {
        println!("Error: No config file found at default paths \
(/usr/etc/supervisord.conf, /usr/supervisord.conf, supervisord.conf, etc/supervisord.conf, /etc/supervisord.conf, /etc/supervisor/supervisord.conf); \
use the -c option to specify a config file at a different path");
        forhelp();
    }
    // can defer Args
    exec()
}

fn forhelp() {
    println!("For help, use /usr/bin/taskmasterd -h");
    exit(2);
}
