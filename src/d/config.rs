use serde::de::{self, Deserializer};
use serde::Deserialize;
use serde_ini;
use serde_yaml::Value;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Config {
    unix_http_server: UnixHttpServer,
    supervisord: Supervisord,
    #[serde(rename = "rpcinterface:supervisor")]
    rpcinterface_supervisor: RpcInterfaceSupervisor,
    supervisorctl: SupervisorCtl,
    include: Include,
    inet_http_server: InetHttpServer,
}

#[derive(Debug, Deserialize)]
pub struct UnixHttpServer {
    file: String,
    chmod: String,
}

#[derive(Debug, Deserialize)]
pub struct Supervisord {
    #[serde(deserialize_with = "deserialize_nodaemon")]
    nodaemon: bool,
    logfile: String,
    pidfile: String,
    childlogdir: String,
}

fn deserialize_nodaemon<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the value as a dynamic type
    let value: Value = Deserialize::deserialize(deserializer)?;

    // Try to convert the value to a boolean
    if let Some(b) = value.as_bool() {
        Ok(b)
    } else if let Some(s) = value.as_str() {
        match s {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(de::Error::custom("Invalid value for nodaemon field")),
        }
    } else {
        Err(de::Error::custom("Invalid value for nodaemon field"))
    }
}

#[derive(Debug, Deserialize)]
pub struct RpcInterfaceSupervisor {
    #[serde(rename = "supervisor.rpcinterface_factory")]
    supervisor_rpcinterface_factory: String,
}

#[derive(Debug, Deserialize)]
pub struct SupervisorCtl {
    serverurl: String,
}

#[derive(Debug, Deserialize)]
pub struct Include {
    files: String,
}

#[derive(Debug, Deserialize)]
pub struct InetHttpServer {
    port: String,
    username: String,
    password: String,
}

pub fn parse_yq_file(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // Read the contents of the YAML/JSON file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the YAML/JSON contents into the Config struct
    let config: Config = serde_yaml::from_str(&contents)?;

    Ok(config)
}

pub fn parse_ini_file(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // Read the contents of the INI file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
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
