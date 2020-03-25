pub mod capabilities;
pub mod config;
pub mod group;
pub mod light;
pub mod scene;

use std::net::IpAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    Discover(Discover),
    Register(Register),
    Config(Config),
    Light(Light),
    Group(Group),
    Scene(Scene),
    Capabilities(Capabilities),
}

/// Discovers bridges in the local network
#[derive(Debug, StructOpt)]
pub struct Discover {}

/// Registers a new user on a bridge
#[derive(Debug, StructOpt)]
pub struct Register {
    /// IP address of the bridge, if omitted the user will be registered on the first discovered
    /// bridge
    pub ip_address: Option<IpAddr>,
    /// Sets environment variables for the current session
    #[structopt(long, short)]
    pub set_env: bool,
}

/// Modifies or prints the bridge configuration
#[derive(Debug, StructOpt)]
pub enum Config {
    Set(config::Set),
    Get(config::Get),
}

/// Modifies, prints, searches or deletes lights
#[derive(Debug, StructOpt)]
pub enum Light {
    Set(light::Set),
    Get(light::Get),
    Search(light::Search),
    Delete(light::Delete),
}

/// Modifies, prints, creates or deletes groups
#[derive(Debug, StructOpt)]
pub enum Group {
    Set(group::Set),
    Get(group::Get),
    Create(group::Create),
    Delete(group::Delete),
}

/// Modifies, prints, creates or deletes scenes
#[derive(Debug, StructOpt)]
pub enum Scene {
    Set(scene::Set),
    Get(scene::Get),
    Create(scene::Create),
    Delete(scene::Delete),
}

/// Prints capabilities of resources
#[derive(Debug, StructOpt)]
pub enum Capabilities {
    Get(capabilities::Get),
}
