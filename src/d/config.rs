use serde::Deserialize;
use serde_ini;
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
    nodaemon: bool,
    logfile: String,
    pidfile: String,
    childlogdir: String,
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

    // Deserialize the INI contents into the Config struct
    let config: Config = serde_ini::from_str(&contents)?;

    Ok(config)
}
