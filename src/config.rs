use serde::de::{self, Deserializer};
use serde::Deserialize;
use serde_ini;
use serde_yaml::Value;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    unix_http_server: Option<UnixHttpServer>,
    #[serde(default = "default_supervisord")]
    supervisord: Supervisord,
    #[serde(rename = "rpcinterface:supervisor", default)]
    rpcinterface_supervisor: Option<RpcInterfaceSupervisor>,
    #[serde(default)]
    supervisorctl: Option<SupervisorCtl>,
    #[serde(default)]
    include: Option<Include>,
    #[serde(default)]
    inet_http_server: Option<InetHttpServer>,
}

#[derive(Debug, Deserialize)]
pub struct UnixHttpServer {
    #[serde(default)]
    file: Option<String>,
    #[serde(default)]
    chmod: Option<String>,
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

fn default_user() -> String {
    if let Ok(user) = env::var("USER") {
        user
    } else {
        eprintln!("Could not find which user to use");
        exit(2)
    }
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
    #[serde(default)]
    serverurl: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Include {
    #[serde(default)]
    files: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InetHttpServer {
    #[serde(default)]
    port: Option<String>,
    #[serde(default)]
    username: Option<String>,
    #[serde(default)]
    password: Option<String>,
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

fn default_supervisord() -> Supervisord {
    println!("Error: .ini file does not include taskmasterd section\nFor help, use /usr/bin/taskmasterd -h"); // Should be dynamic ? could be different path could be .ini but also json or yaml
    exit(1)
}
