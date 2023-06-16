use serde::de::{self, Deserializer};
use serde::Deserialize;
use serde_ini;
use serde_yaml::Value;
use std::fs::File;
use std::io::Read;
use std::process;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    unix_http_server: Option<UnixHttpServer>,
    #[serde(default = "default_nodaemon")]
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
    #[serde(deserialize_with = "deserialize_nodaemon")]
    #[serde(default)]
    nodaemon: Option<bool>,
    #[serde(default)]
    logfile: Option<String>,
    #[serde(default)]
    pidfile: Option<String>,
    #[serde(default)]
    childlogdir: Option<String>,
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

fn deserialize_nodaemon<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the value as a dynamic type
    let value: Value = Deserialize::deserialize(deserializer)?;

    // Try to convert the value to a boolean
    if let Some(s) = value.as_str() {
        match s {
            "true" => Ok(Some(true)),
            "false" => Ok(Some(false)),
            _ => Err(de::Error::custom("Invalid value for nodaemon field")),
        }
    } else {
        Ok(None)
    }
}

fn default_nodaemon() -> Supervisord {
    println!("Error: .ini file does not include taskmasterd section\nFor help, use /usr/bin/taskmasterd -h"); // Should be dynamic ?
                                                                                                              // could be different path
                                                                                                              // could be .ini but also json or yaml
    process::exit(1)
}
