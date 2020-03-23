use crate::arg::subcommand;
use crate::config;
use crate::util;

pub fn discover() {
    let ip_addresses = match huelib::bridge::discover() {
        Ok(v) => v,
        Err(e) => util::print_err("Failed to discover bridges", e),
    };
    for i in ip_addresses {
        println!("{}", i);
    }
}

pub fn register(arg: subcommand::Register) {
    let user = match huelib::bridge::register_user(arg.ip_address, "huectl-rs", false) {
        Ok(v) => v,
        Err(e) => util::print_err("Failed to register user", e),
    };
    let ip_address = arg.ip_address.to_string();
    if arg.set_env {
        std::env::set_var(config::VAR_BRIDGE_IP, ip_address);
        std::env::set_var(config::VAR_BRIDGE_USERNAME, user.name);
    } else {
        println!("{}={}", config::VAR_BRIDGE_IP, ip_address);
        println!("{}={}", config::VAR_BRIDGE_USERNAME, user.name);
    }
}
