use crate::{arg::subcommand, config};

pub fn discover(_arg: subcommand::Discover) {
    let ip_addresses = match huelib::bridge::discover() {
        Ok(v) => v,
        Err(e) => exit!("Failed to discover bridges", e),
    };
    for i in ip_addresses {
        println!("{}", i);
    }
}

pub fn register(arg: subcommand::Register) {
    let ip_address = match arg.ip_address {
        Some(v) => v,
        None => match huelib::bridge::discover() {
            Ok(mut v) => match v.pop() {
                Some(v) => v,
                None => exit!("No bridges were found"),
            },
            Err(e) => exit!("Failed to discover bridges", e),
        },
    };
    let user = match huelib::bridge::register_user(ip_address, "huectl-rs", false) {
        Ok(v) => v,
        Err(e) => exit!(
            format!(
                "Failed to register user on bridge with the IP address '{}'",
                ip_address
            ),
            e
        ),
    };
    if arg.set_env {
        std::env::set_var(config::VAR_BRIDGE_IP, ip_address.to_string());
        std::env::set_var(config::VAR_BRIDGE_USERNAME, user.name);
    } else {
        println!("{}={}", config::VAR_BRIDGE_IP, ip_address);
        println!("{}={}", config::VAR_BRIDGE_USERNAME, user.name);
    }
}
