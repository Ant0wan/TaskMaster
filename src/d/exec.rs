use std::process::exit;

enum ProcessState {
    STOPPED,  // The process has been stopped due to a stop request or has never been started.
    STARTING, // The process is starting due to a start request.
    RUNNING,  // The process is running.
    BACKOFF, // The process entered the STARTING state but subsequently exited too quickly (before the time defined in startsecs) to move to the RUNNING state.
    STOPPING, // The process is stopping due to a stop request.
    EXITED,  // The process exited from the RUNNING state (expectedly or unexpectedly).
    FATAL,   // The process could not be started successfully.
    UNKNOWN, // The process is in an unknown state (supervisord programming error).
}

struct ProcessInfo {
    name: String,                             // Name of the process
    state: ProcessState,                      // Current state of the process
    pid: u32,                                 // Process ID
    cpu_usage: f32,                           // CPU usage of the process
    memory_usage: u64,                        // Memory usage of the process
    start_time: DateTime<Utc>,                // Start time of the process
    restart_count: u32,                       // Number of times the process has been restarted
    last_restart_time: Option<DateTime<Utc>>, // Timestamp of the last process restart
    exit_code: Option<u32>,                   // Exit code of the process if it has exited
    supervisor: String,                       // Name or identifier of the process supervisor
    stdout_log_file: Option<String>,          // Path to the stdout log file of the process
    stderr_log_file: Option<String>,          // Path to the stderr log file of the process
    environment: HashMap<String, String>,     // Environment variables specific to the process
    program: Program,                         // Program elements
}

impl ProcessState {
    fn transition(&self, process: &ProcessInfo) -> Option<ProcessState> {
        match self {
            ProcessState::STOPPED => Some(ProcessState::STARTING), // should implement UNKNOW ?
            ProcessState::STARTING => {
                if process.backoff {
                    Some(ProcessState::BACKOFF)
                } else if process.running {
                    Some(ProcessState::RUNNING)
                } else if process.stopping {
                    Some(ProcessState::STOPPING)
                } else {
                    Some(ProcessState::UNKNOWN)
                }
            }
            ProcessState::RUNNING => {
                if process.stoppable {
                    Some(ProcessState::STOPPING)
                } else if process.exit {
                    Some(ProcessState::EXITED)
                } else {
                    Some(ProcessState::UNKNOWN)
                }
            }
            ProcessState::BACKOFF => {
                if process.starting {
                    Some(ProcessState::STARTING)
                } else if process.fatal {
                    Some(ProcessState::FATAL)
                } else {
                    Some(ProcessState::UNKNOWN)
                }
            }
            ProcessState::STOPPING => Some(ProcessState::STOPPED), // should implement UNKNOWN ?
            ProcessState::EXITED => Some(ProcessState::STARTING),
            ProcessState::FATAL => Some(ProcessState::STARTING),
            ProcessState::UNKNOWN => exit(2), // + error message,
        }
    }
}

pub fn exec() {
    println!("Hello world !");
    exit(0);
}
