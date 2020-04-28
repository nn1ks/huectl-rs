use crate::{output::Config as OutputConfig, util};
use huelib::resource::{config, Modifier};
use std::net::IpAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Arg {
    /// Modifies attributes of the bridge configuration
    Set(Set),
    /// Prints the bridge configuration
    Get,
}

#[derive(Debug, StructOpt)]
pub struct Set {
    /// Sets the name of the bridge
    #[structopt(long, short)]
    pub name: Option<String>,
    /// Sets the ip address of the bridge
    #[structopt(long, short)]
    pub ip_address: Option<IpAddr>,
    /// Sets the network mask of the bridge
    #[structopt(long)]
    pub netmask: Option<String>,
    /// Sets the gateway ip address of the bridge
    #[structopt(long, short)]
    pub gateway: Option<IpAddr>,
    /// Activates DHCP for the bridge
    #[structopt(long, short)]
    pub dhcp: bool,
    /// Deactivates DHCP for the bridge
    #[structopt(long, short = "D")]
    pub no_dhcp: bool,
    /// Sets the proxy port of the bridge
    #[structopt(long)]
    pub proxy_port: Option<u16>,
    /// Sets the proxy address of the bridge
    #[structopt(long)]
    pub proxy_address: Option<IpAddr>,
    /// Deactives the proxy on the bridge
    #[structopt(long)]
    pub no_proxy: bool,
    /// Adds the closest lamp to the ZigBee network
    #[structopt(long, short)]
    pub touchlink: bool,
    /// Sets the ZigBee channel
    #[structopt(long, short)]
    pub zigbee_channel: Option<u8>,
    /// Sets the current time of the bridge
    #[structopt(long)]
    pub current_time: Option<String>,
    /// Sets the timezone of the bridge
    #[structopt(long)]
    pub timezone: Option<String>,
}

impl Set {
    pub fn to_modifier(&self) -> config::Modifier {
        let mut modifier = config::Modifier::new();
        if let Some(v) = &self.name {
            modifier = modifier.name(v);
        }
        if let Some(v) = self.ip_address {
            modifier = modifier.ip_address(v);
        }
        if let Some(v) = &self.netmask {
            modifier = modifier.netmask(v);
        }
        if let Some(v) = self.gateway {
            modifier = modifier.gateway(v);
        }
        if self.dhcp {
            modifier = modifier.dhcp(true);
        } else if self.no_dhcp {
            modifier = modifier.dhcp(false);
        }
        if let Some(v) = self.proxy_port {
            modifier = modifier.proxy_port(v);
        }
        if let Some(v) = self.proxy_address {
            modifier = modifier.proxy_address(Some(v));
        }
        if self.no_proxy {
            modifier = modifier.proxy_address(None);
            modifier = modifier.proxy_port(0);
        }
        if self.touchlink {
            modifier = modifier.touchlink();
        }
        if let Some(v) = self.zigbee_channel {
            modifier = modifier.zigbee_channel(v);
        }
        if let Some(v) = &self.current_time {
            modifier = modifier.current_time(v);
        }
        if let Some(v) = &self.timezone {
            modifier = modifier.timezone(v);
        }
        modifier
    }
}

pub fn set(arg: Set) {
    let responses = match util::get_bridge().set_config(&arg.to_modifier()) {
        Ok(v) => v,
        Err(e) => exit!("Failed to set config", e),
    };
    for i in responses {
        println!("{}", i);
    }
}

pub fn get() {
    let bridge = util::get_bridge();
    match bridge.get_config() {
        Ok(v) => println!(
            "{}",
            serde_json::to_string_pretty(&OutputConfig::from(v)).unwrap()
        ),
        Err(e) => exit!("Failed to get config", e),
    };
}
