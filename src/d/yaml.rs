use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct YamlConfig {
    unix_http_server: UnixHttpServer,
    supervisord: Supervisord,
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
