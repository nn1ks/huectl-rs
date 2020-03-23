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
    create-group    Creates a group
    create-scene    Creates a scene
    delete-group    Deletes a group
    delete-light    Deletes a light
    delete-scene    Deletes a scene
    discover        Discovers bridges in the local network
    get-config      Prints the bridge configuration
    get-group       Prints the state and attributes of a group
    get-light       Prints the state and attributes of a light
    get-scene       Prints the state and attributes of a scene
    help            Prints this message or the help of the given subcommand(s)
    register        Registers a new user on a bridge
    search-light    Searches for new lights
    set-config      Modifies attributes of the bridge configuration
    set-group       Modifies the state and attributes of a group
    set-light       Modifies the state and attributes of a light
    set-scene       Modifies the state and attributes of a scene
```
