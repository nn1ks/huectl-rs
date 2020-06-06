# huectl-rs

[![Build](https://img.shields.io/github/workflow/status/yuqio/huectl-rs/CI)](https://github.com/yuqio/huectl-rs/actions)
[![Version](https://img.shields.io/github/v/release/yuqio/huectl-rs?color=orange)](https://github.com/yuqio/huectl-rs/releases)
[![License](https://img.shields.io/github/license/yuqio/huectl-rs?color=yellow)](https://github.com/yuqio/huectl-rs/blob/master/LICENSE)

A command line interface to Philips Hue using the [huelib](https://github.com/yuqio/huelib-rs) crate.

## Installation

### Building from source

```sh
git clone https://github.com/yuqio/huectl-rs
cd huectl-rs
cargo install --path .
```

## Configuration

Environment variables are used for configuration.

- `HUECTL_BRIDGE_IP`: The IP address of the bridge
- `HUECTL_BRIDGE_USERNAME`: The name of a registered user on the bridge

## Usage

```
USAGE:
    huectl <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    config          Modifies or prints the bridge configuration
    discover        Discovers bridges in the local network
    group           Modifies, prints, creates or deletes groups
    help            Prints this message or the help of the given subcommand(s)
    light           Modifies, prints, searches or deletes lights
    register        Registers a new user on a bridge
    resourcelink    Modifier, prints, creates or deletes resourcelinks
    rule            Modifier, prints, creates or deletes rules
    scene           Modifies, prints, creates or deletes scenes
    schedule        Modifies, prints, creates or deletes schedules
    sensor          Modifies, prints, searches or deletes sensors
```
