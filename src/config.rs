use envconfig::Envconfig;
use std::net::IpAddr;

pub const VAR_BRIDGE_IP: &str = "HUECTL_BRIDGE_IP";
pub const VAR_BRIDGE_USERNAME: &str = "HUECTL_BRIDGE_USERNAME";

pub fn get() -> Result<Config, envconfig::Error> {
    Config::init()
}

#[derive(Debug, Envconfig)]
pub struct Config {
    #[envconfig(from = "HUECTL_BRIDGE_IP")]
    pub bridge_ip: IpAddr,
    #[envconfig(from = "HUECTL_BRIDGE_USERNAME")]
    pub bridge_username: String,
}
