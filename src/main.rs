#[macro_use]
extern crate envconfig_derive;

mod arg;
mod command;
mod config;
mod util;

use arg::subcommand::{self, Subcommand};
use structopt::StructOpt;

fn main() {
    let args = arg::Args::from_args();
    match args.subcommand {
        Subcommand::Discover(_) => command::bridge::discover(),
        Subcommand::Register(v) => command::bridge::register(v),
        Subcommand::Config(v) => match v {
            subcommand::Config::Set(v) => command::config::set(v),
            subcommand::Config::Get(v) => command::config::get(v),
        },
        Subcommand::Light(v) => match v {
            subcommand::Light::Set(v) => command::light::set(v),
            subcommand::Light::Get(v) => command::light::get(v),
            subcommand::Light::Search(v) => command::light::search(v),
            subcommand::Light::Delete(v) => command::light::delete(v),
        },
        Subcommand::Group(v) => match v {
            subcommand::Group::Set(v) => command::group::set(v),
            subcommand::Group::Get(v) => command::group::get(v),
            subcommand::Group::Create(v) => command::group::create(v),
            subcommand::Group::Delete(v) => command::group::delete(v),
        },
        Subcommand::Scene(v) => match v {
            subcommand::Scene::Set(v) => command::scene::set(v),
            subcommand::Scene::Get(v) => command::scene::get(v),
            subcommand::Scene::Create(v) => command::scene::create(v),
            subcommand::Scene::Delete(v) => command::scene::delete(v),
        },
        Subcommand::Capabilities(v) => match v {
            subcommand::Capabilities::Get(v) => command::capabilities::get(v),
        },
    };
}
