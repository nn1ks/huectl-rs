use crate::{arg::subcommand, util};
use std::fmt;

struct Config(huelib::Config);

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        // TODO: Create struct for printing
        output.push_str(&format!("{:#?}", self.0));
        write!(f, "{}", output)
    }
}

pub fn set(arg: subcommand::config::Set) {
    let responses = match util::get_bridge().set_config(&arg.to_modifier()) {
        Ok(v) => v,
        Err(e) => exit!("Failed to set config", e),
    };
    for i in responses {
        println!("{}", i);
    }
}

pub fn get(_arg: subcommand::config::Get) {
    let bridge = util::get_bridge();
    match bridge.get_config() {
        Ok(v) => println!("{}", Config(v)),
        Err(e) => exit!("Failed to get scene", e),
    };
}
