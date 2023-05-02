use std::env;

pub fn print_usage() {
    println!("Usage: /usr/bin/taskmasterctl [options] [action [arguments]]");
    println!("\nOptions:");
    println!("-c/--configuration FILENAME -- configuration file path (searches if not given)");
    println!("-h/--help -- print usage message and exit");
    println!("-i/--interactive -- start an interactive shell after executing commands");
    println!("-s/--serverurl URL -- URL on which taskmasterd server is listening (default \"http://localhost:9001\")");
    println!("-u/--username USERNAME -- username to use for authentication with server");
    println!("-p/--password PASSWORD -- password to use for authentication with server");
    println!("-r/--history-file -- keep a readline history (if readline is available)");
    println!("\nActions are commands like \"tail\" or \"stop\". If -i is specified or no action is specified on the command line, a \"shell\" interpreting actions typed interactively is started. Use the action \"help\" to find out about available actions.");
}

pub struct Args {
    pub configuration: Option<String>,
    pub help: bool,
    pub interactive: bool,
    pub serverurl: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub history_file: bool,
    pub action: Option<String>,
    pub arguments: Vec<String>,
}

impl Args {
    fn new() -> Self {
        Args {
            configuration: None,
            help: false,
            interactive: false,
            serverurl: Some(String::from("http://localhost:9001")),
            username: None,
            password: None,
            history_file: false,
            action: None,
            arguments: Vec::new(),
        }
    }
}

pub fn parse_args() -> Args {
    let mut args = Args::new();

    let mut args_iter = env::args().skip(1);
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-c" | "--configuration" => {
                args.configuration =
                    Some(args_iter.next().expect("missing configuration file path"));
            }
            "-h" | "--help" => {
                args.help = true;
            }
            "-i" | "--interactive" => {
                args.interactive = true;
            }
            "-s" | "--serverurl" => {
                args.serverurl = Some(args_iter.next().expect("missing server URL"));
            }
            "-u" | "--username" => {
                args.username = Some(args_iter.next().expect("missing username"));
            }
            "-p" | "--password" => {
                args.password = Some(args_iter.next().expect("missing password"));
            }
            "-r" | "--history-file" => {
                args.history_file = true;
            }
            _ => {
                if args.action.is_none() {
                    args.action = Some(arg);
                } else {
                    args.arguments.push(arg);
                }
            }
        }
    }

    args
}
