#[macro_use]
extern crate envconfig_derive;

#[macro_use]
mod util;

mod arg;
mod config;

fn main() {
    arg::exec();
}
