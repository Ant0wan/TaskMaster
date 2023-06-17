use std::env;

pub fn print_usage() {
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

pub struct Args {
    pub configuration: Option<String>,
    pub nodaemon: bool,
    pub silent: bool,
    pub help: bool,
    pub version: bool,
    pub user: Option<String>,
    pub umask: Option<String>,
    pub directory: Option<String>,
    pub logfile: Option<String>,
    pub logfile_maxbytes: Option<String>,
    pub logfile_backups: Option<String>,
    pub loglevel: Option<String>,
    pub pidfile: Option<String>,
    pub identifier: Option<String>,
    pub childlogdir: Option<String>,
    pub nocleanup: bool,
    pub minfds: Option<String>,
    pub strip_ansi: bool,
    pub minprocs: Option<u32>,
    pub profile_options: Option<String>,
}

impl Args {
    fn new() -> Self {
        Args {
            configuration: None,
            nodaemon: false,
            silent: false,
            help: false,
            version: false,
            user: None,
            umask: None,
            directory: None,
            logfile: None,
            logfile_maxbytes: None,
            logfile_backups: None,
            loglevel: None,
            pidfile: None,
            identifier: None,
            childlogdir: None,
            nocleanup: false,
            minfds: None,
            strip_ansi: false,
            minprocs: None,
            profile_options: None,
        }
    }
}

pub fn parse_args() -> Args {
    let mut args: Args = Args::new();

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
            "-u" | "--user" => {
                args.user = Some(args_iter.next().expect("missing user or numeric uid"));
            }
            "-m" | "--umask" => {
                args.umask = Some(
                    args_iter
                        .next()
                        .expect("missing umask for daemon subprocess"),
                );
            }
            "-d" | "--directory" => {
                args.directory = Some(args_iter.next().expect("missing directory to chdir to"));
            }
            "-l" | "--logfile" => {
                args.logfile = Some(args_iter.next().expect("missing logfile path"));
            }
            "-y" | "--logfile_maxbytes" => {
                args.logfile_maxbytes =
                    Some(args_iter.next().expect("missing max size of logfile"));
            }
            "-z" | "--logfile_backups" => {
                args.logfile_backups = Some(
                    args_iter
                        .next()
                        .expect("missing number of backups for logfile"),
                );
            }
            "-e" | "--loglevel" => {
                args.loglevel = Some(args_iter.next().expect("missing log level"));
            }
            "-j" | "--pidfile" => {
                args.pidfile = Some(args_iter.next().expect("missing pid file path"));
            }
            "-i" | "--identifier" => {
                args.identifier = Some(args_iter.next().expect("missing identifier"));
            }
            "-q" | "--childlogdir" => {
                args.childlogdir = Some(
                    args_iter
                        .next()
                        .expect("missing log directory for child process logs"),
                );
            }
            "-k" | "--nocleanup" => {
                args.nocleanup = true;
            }
            "-a" | "--minfds" => {
                args.minfds = Some(
                    args_iter
                        .next()
                        .expect("missing minimum number of file descriptors"),
                );
            }
            "-t" | "--strip_ansi" => {
                args.strip_ansi = true;
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
                eprintln!(
                    "Error: option {} not recognized\nFor help, use /usr/bin/taskmasterd -h",
                    &arg[0..2]
                );
            }
        }
    }

    args
}
