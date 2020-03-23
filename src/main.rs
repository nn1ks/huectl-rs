#[macro_use]
extern crate envconfig_derive;

mod arg;
mod command;
mod config;
mod util;

use arg::subcommand::Subcommand;
use structopt::StructOpt;

fn main() {
    let args = arg::Args::from_args();
    match args.subcommand {
        Subcommand::Discover(_) => command::bridge::discover(),
        Subcommand::Register(v) => command::bridge::register(v),
        Subcommand::SetConfig(v) => command::config::set(v),
        Subcommand::GetConfig(v) => command::config::get(v),
        Subcommand::SetLight(v) => command::light::set(v),
        Subcommand::GetLight(v) => command::light::get(v),
        Subcommand::SearchLight(v) => command::light::search(v),
        Subcommand::DeleteLight(v) => command::light::delete(v),
        Subcommand::CreateGroup(v) => command::group::create(v),
        Subcommand::SetGroup(v) => command::group::set(v),
        Subcommand::GetGroup(v) => command::group::get(v),
        Subcommand::DeleteGroup(v) => command::group::delete(v),
        Subcommand::CreateScene(v) => command::scene::create(v),
        Subcommand::SetScene(v) => command::scene::set(v),
        Subcommand::GetScene(v) => command::scene::get(v),
    };
}
