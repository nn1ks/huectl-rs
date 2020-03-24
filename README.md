# huectl-rs

[![Version](https://img.shields.io/github/v/release/yuqio/huectl-rs?color=orange)](https://github.com/yuqio/huectl-rs/releases)
[![License](https://img.shields.io/github/license/yuqio/huectl-rs)](https://github.com/yuqio/huectl-rs/blob/master/LICENSE)
[![Code size](https://img.shields.io/github/languages/code-size/yuqio/huectl-rs)]()
[![Lines of code](https://tokei.rs/b1/github/yuqio/huectl-rs?category=code)]()

A command line interface to Philips Hue using the [huelib](https://github.com/yuqio/huelib-rs) crate.

## Installation

### Building from source

```sh
git clone https://github.com/yuqio/huectl-rs
cd huectl-rs
cargo install --release
```

## Usage

```
USAGE:
    huectl <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    capabilities    Prints capabilities of resources
    config          Modifies or prints the bridge configuration
    discover        Discovers bridges in the local network
    group           Modifies, prints, creates or deletes groups
    help            Prints this message or the help of the given subcommand(s)
    light           Modifies, prints, searches or deletes lights
    register        Registers a new user on a bridge
    scene           Modifies, prints, creates or deletes scenes
```
