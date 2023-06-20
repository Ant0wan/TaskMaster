use serde::de;
use serde::de::Deserializer;
use serde::Deserialize;
use serde_ini;
use serde_yaml::Value;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;
use users::get_current_username;
use users::get_group_by_name;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    unix_http_server: Option<UnixHttpServer>,
    #[serde(default)]
    supervisord: Option<Supervisord>,
    #[serde(rename = "rpcinterface:supervisor", default)]
    rpcinterface_supervisor: Option<RpcInterfaceSupervisor>,
    #[serde(default)]
    supervisorctl: Option<SupervisorCtl>,
    #[serde(default)]
    include: Option<Include>,
    #[serde(default)]
    inet_http_server: Option<InetHttpServer>,
    #[serde(flatten)]
    //program: Option<HashMap<String, Program>>,
    program: HashMap<String, Program>,
} // This has to have some combination of options true or false depending whether supervisord or supervosirctl read the config

#[derive(Debug, Deserialize)]
pub struct Program {
    #[serde(default)]
    pub command: String,
    #[serde(default = "default_process_name")]
    pub process_name: String,
    //#[serde(default = "default_numprocs")]
    //pub numprocs: u32,
    //#[serde(default = "default_numprocs_start")]
    //pub numprocs_start: u32,
    //#[serde(default = "default_priority")]
    //pub priority: u32,
    #[serde(default = "default_true")]
    #[serde(deserialize_with = "deserialize_bool")]
    pub autostart: bool,
    //#[serde(default = "default_startsecs")]
    //pub startsecs: u32,
    //#[serde(default = "default_startretries")]
    //pub startretries: u32,
    #[serde(default = "default_autorestart")]
    pub autorestart: Restart,
    //#[serde(default = "default_exitcodes")]
    //pub exitcodes: Vec<u32>,
    #[serde(default = "default_stopsignal")]
    pub stopsignal: StopSignal,
    //    #[serde(default = "default_stopwaitsecs")]
    //    pub stopwaitsecs: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default = "default_false")]
    pub stopasgroup: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default = "default_false")]
    pub killasgroup: bool,
    #[serde(default = "default_user")]
    pub user: String,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default = "default_false")]
    pub redirect_stderr: bool,
    //
    //#[serde(default)]
    //pub stdout_logfile: Option<String>,
    //    #[serde(default)]
    //    pub stdout_logfile_maxbytes: Option<String>,
    //    #[serde(default)]
    //    pub stdout_logfile_backups: Option<u32>,
    //    #[serde(default)]
    //    pub stderr_logfile: Option<String>,
    //    #[serde(default)]
    //    pub stderr_logfile_maxbytes: Option<String>,
    //    #[serde(default)]
    //    pub stderr_logfile_backups: Option<u32>,
    //    #[serde(default)]
    //    pub environment: Option<HashMap<String, String>>,
}

fn default_stopwaitsecs() -> u32 {
    10
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Restart {
    False,
    Never,
    Always,
    True,
    Unexpected,
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum StopSignal {
    TERM,
    HUP,
    INT,
    QUIT,
    KILL,
    USR1,
    USR2,
}

fn default_autorestart() -> Restart {
    Restart::Unexpected
}

fn default_stopsignal() -> StopSignal {
    StopSignal::TERM
}

fn default_priority() -> u32 {
    999
}

fn default_exitcodes() -> Vec<u32> {
    vec![0]
}

fn default_numprocs_start() -> u32 {
    0
}

fn default_numprocs() -> u32 {
    1
}

fn default_startretries() -> u32 {
    3
}

fn default_startsecs() -> u32 {
    1
}

fn default_process_name() -> String {
    String::from("%(program_name)s")
}

#[derive(Debug, Deserialize)]
pub struct UnixHttpServer {
    #[serde(default)]
    pub file: Option<String>,
    #[serde(default = "default_chmod")]
    pub chmod: u32,
    #[serde(default = "default_chown")]
    pub chown: String,
    #[serde(default = "default_user")]
    pub username: String,
    #[serde(default)]
    pub password: Option<String>,
}

fn default_chmod() -> u32 {
    0o700
}

#[derive(Debug, Deserialize)]
pub struct Supervisord {
    #[serde(default = "default_logfile")]
    pub logfile: String,
    #[serde(default = "default_logfile_maxbytes")]
    pub logfile_maxbytes: String,
    #[serde(default = "default_logfile_backups")]
    pub logfile_backups: u32,
    #[serde(default = "default_loglevel")]
    pub loglevel: String,
    #[serde(default = "default_pidfile")]
    pub pidfile: String,
    #[serde(default = "default_umask")]
    pub umask: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default = "default_false")]
    pub nodaemon: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default = "default_false")]
    pub silent: bool,
    #[serde(default = "default_minfds")]
    pub minfds: u32,
    #[serde(default = "default_minprocs")]
    pub minprocs: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default = "default_false")]
    pub nocleanup: bool,
    #[serde(default = "default_childlogdir")]
    pub childlogdir: String,
    #[serde(default = "default_user")]
    pub user: String,
    #[serde(default = "default_directory")]
    pub directory: String,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default = "default_false")]
    pub strip_ansi: bool,
    #[serde(default)]
    pub environment: Option<String>,
    #[serde(default = "default_identifier")]
    pub identifier: String,
}

fn default_directory() -> String {
    if let Ok(dir) = env::current_dir() {
        dir.to_string_lossy().into_owned()
    } else {
        eprintln!("Could not find current directory");
        exit(2)
    }
}

fn default_chown() -> String {
    format!("{}:{}", default_user(), default_group())
}

fn default_user() -> String {
    if let Some(user) = get_current_username() {
        if let Some(user_name) = user.to_str() {
            return user_name.to_string();
        }
    }
    eprintln!("Could not find which user to use");
    exit(2);
}

fn default_group() -> String {
    if let Some(username) = get_current_username() {
        if let Some(group) = get_group_by_name(&username) {
            if let Some(groupname) = group.name().to_str() {
                return groupname.to_owned();
            }
        }
    }
    eprintln!("Could not find which group to use");
    exit(2);
}

fn default_childlogdir() -> String {
    let temp_dir: PathBuf = env::temp_dir();
    temp_dir.to_string_lossy().into_owned()
}

fn default_pidfile() -> String {
    String::from("$CWD/supervisord.pid")
}

fn default_logfile() -> String {
    String::from("$CWD/supervisord.log")
}

fn default_logfile_maxbytes() -> String {
    String::from("50MB")
}

fn default_logfile_backups() -> u32 {
    10
}

fn default_loglevel() -> String {
    String::from("info")
}

fn default_umask() -> u32 {
    0o022
}

fn default_minfds() -> u32 {
    1024
}

fn default_minprocs() -> u32 {
    200
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_identifier() -> String {
    String::from("supervisor")
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the value as a dynamic type
    let value: Value = Deserialize::deserialize(deserializer)?;

    // Try to convert the value to a boolean
    if let Some(s) = value.as_str() {
        match s {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(de::Error::custom("Invalid value for a boolean field")),
        }
    } else {
        Ok(false)
    }
}

#[derive(Debug, Deserialize)]
pub struct RpcInterfaceSupervisor {
    #[serde(rename = "supervisor.rpcinterface_factory")]
    #[serde(default)]
    supervisor_rpcinterface_factory: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SupervisorCtl {
    #[serde(default = "default_serverurl")]
    serverurl: String,
    #[serde(default)]
    username: Option<String>,
    #[serde(default)]
    password: Option<String>,
    #[serde(default = "default_identifier")]
    prompt: String,
    #[serde(default)]
    history_file: Option<String>,
}

fn default_serverurl() -> String {
    String::from("http://localhost:9001")
}

#[derive(Debug, Deserialize)]
pub struct Include {
    #[serde(default)]
    files: String,
}

#[derive(Debug, Deserialize)]
pub struct InetHttpServer {
    #[serde(default = "default_port")]
    port: String,
    #[serde(default)]
    username: Option<String>,
    #[serde(default)]
    password: Option<String>,
}

fn default_port() -> String {
    println!("Error: .ini file, InetHttpServer section does not include a valid port value\nFor help, use /usr/bin/taskmasterd -h"); // Should be dynamic ? Check the supervisord error message. could be different path could be .ini but also json or yaml
    exit(1)
}

pub fn parse_yq_file(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // Read the contents of the YAML/JSON file
    let mut file: File = File::open(filename)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the YAML/JSON contents into the Config struct
    let config: Config = serde_yaml::from_str(&contents)?;

    Ok(config)
}

pub fn parse_ini_file(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // Read the contents of the INI file
    let mut file: File = File::open(filename)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    // Remove inline comments from the INI contents
    let cleaned_contents = remove_inline_comments(&contents);

    // Deserialize the cleaned INI contents into the Config struct
    let config: Config = serde_ini::from_str(&cleaned_contents)?;

    Ok(config)
}

fn remove_inline_comments(contents: &str) -> String {
    contents
        .lines()
        .map(|line| {
            if let Some(position) = line.find(';') {
                &line[..position]
            } else {
                line
            }
        })
        .collect::<Vec<&str>>()
        .join("\n")
}

//fn default_supervisord() -> Supervisord {
//    println!("Error: .ini file does not include taskmasterd section\nFor help, use /usr/bin/taskmasterd -h"); // Should be dynamic ? could be different path could be .ini but also json or yaml
//    exit(1)
//}
