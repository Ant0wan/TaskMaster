use std::collections::HashMap;
use std::env;
use std::fs;

use taskmaster::config_parser::{parse_ini_file, IniConfig};

fn print_usage() {
    println!("Usage: /usr/bin/taskmasterd [options]");
    println!("Options:");
    println!("-c/--configuration FILENAME -- configuration file path (searches if not given)");
    println!("-n/--nodaemon -- run in the foreground (same as 'nodaemon=true' in config file)");
    println!("-s/--silent -- no logs to stdout (maps to 'silent=true' in config file)");
    println!("-h/--help -- print this usage message and exit");
    println!("-v/--version -- print taskmasterd version number and exit");
    println!("-u/--user USER -- run taskmasterd as this user (or numeric uid)");
    println!("-m/--umask UMASK -- use this umask for daemon subprocess (default is 022)");
    println!("-d/--directory DIRECTORY -- directory to chdir to when daemonized");
    println!("-l/--logfile FILENAME -- use FILENAME as logfile path");
    println!("-y/--logfile_maxbytes BYTES -- use BYTES to limit the max size of logfile");
    println!("-z/--logfile_backups NUM -- number of backups to keep when max bytes reached");
    println!("-e/--loglevel LEVEL -- use LEVEL as log level (debug,info,warn,error,critical)");
    println!("-j/--pidfile FILENAME -- write a pid file for the daemon process to FILENAME");
    println!("-i/--identifier STR -- identifier used for this instance of taskmasterd");
    println!("-q/--childlogdir DIRECTORY -- the log directory for child process logs");
    println!("-k/--nocleanup -- prevent the process from performing cleanup (removal of");
    println!("                  old automatic child log files) at startup.");
    println!("-a/--minfds NUM -- the minimum number of file descriptors for start success");
    println!("-t/--strip_ansi -- strip ansi escape codes from process output");
    println!("--minprocs NUM -- the minimum number of processes available for start success");
    println!("--profile_options OPTIONS -- run taskmasterd under profiler and output");
    println!("                              results based on OPTIONS, which is a comma-sep'd");
    println!("                              list of 'cumulative', 'calls', and/or 'callers',");
    println!("                              e.g. 'cumulative,callers')");
}

struct Args {
    configuration: Option<String>,
    nodaemon: bool,
    silent: bool,
    help: bool,
    version: bool,
    minprocs: Option<u32>,
    profile_options: Option<String>,
}

impl Args {
    fn new() -> Self {
        Args {
            configuration: None,
            nodaemon: false,
            silent: false,
            help: false,
            version: false,
            minprocs: None,
            profile_options: None,
        }
    }
}

fn parse_args() -> Args {
    let mut args = Args::new();

    let mut args_iter = env::args().skip(1);
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-c" | "--configuration" => {
                args.configuration =
                    Some(args_iter.next().expect("missing configuration file path"));
            }
            "-n" | "--nodaemon" => {
                args.nodaemon = true;
            }
            "-s" | "--silent" => {
                args.silent = true;
            }
            "-h" | "--help" => {
                args.help = true;
            }
            "-v" | "--version" => {
                args.version = true;
            }
            "--minprocs" => {
                args.minprocs = Some(
                    args_iter
                        .next()
                        .expect("missing minimum processes value")
                        .parse()
                        .expect("invalid minimum processes value"),
                );
            }
            "--profile_options" => {
                args.profile_options = Some(args_iter.next().expect("missing profile options"));
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
            }
        }
    }

    args
}

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
