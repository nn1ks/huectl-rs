# huectl-rs

[![Build](https://img.shields.io/github/workflow/status/yuqio/huectl-rs/CI?labelColor=4c566a&color=a3be8c&logo=github)](https://github.com/yuqio/huectl-rs/actions)
[![Crate](https://img.shields.io/crates/v/huectl?labelColor=4c566a&color=81a1c1&logo=rust)](https://crates.io/crates/huectl)
[![License](https://img.shields.io/crates/l/huectl?labelColor=4c566a&color=b48ead)](https://github.com/yuqio/huectl-rs/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/huectl?labelColor=4c566a&color=5e81ac)](https://crates.io/crates/huectl)

A command line interface to Philips Hue using the [huelib](https://github.com/yuqio/huelib-rs) crate.

## Installation

### Building from source

1. Install and setup [Rust](https://www.rust-lang.org)
2. Run `cargo install huectl`

### Pre-compiled binary

1. Download a binary from the [release page](https://github.com/yuqio/huectl-rs/releases)
2. Move the binary to your `PATH`

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
