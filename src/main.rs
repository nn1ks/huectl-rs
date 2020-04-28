#[macro_use]
extern crate envconfig_derive;

#[macro_use]
mod util;

mod arg;
mod config;
mod output;

fn main() {
    arg::exec();
}
