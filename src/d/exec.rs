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

impl ProcessState {
    fn transition(&self, config: &Config) -> Option<ProcessState> {
        match self {
            ProcessState::STOPPED => Some(ProcessState::STARTING), // should implement UNKNOW ?
            ProcessState::STARTING => {
                if config.backoff {
                    Some(ProcessState::BACKOFF)
                } else if config.running {
                    Some(ProcessState::RUNNING)
                } else if config.stopping {
                    Some(ProcessState::STOPPING)
                } else {
                    Some(ProcessState::UNKNOWN)
                }
            }
            ProcessState::RUNNING => {
                if config.stoppable {
                    Some(ProcessState::STOPPING)
                } else if config.exit {
                    Some(ProcessState::EXITED)
                } else {
                    Some(ProcessState::UNKNOWN)
                }
            }
            ProcessState::BACKOFF => {
                if config.starting {
                    Some(ProcessState::STARTING)
                } else if config.fatal {
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
