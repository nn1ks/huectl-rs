use crate::{arg::subcommand, util};
use huelib::Modifier;
use std::fmt;

struct Group(huelib::Group);

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("Group {}:\n", self.0.id));
        output.push_str(&format!("    Name: {:?}\n", self.0.name));
        output.push_str(&format!("    Lights: {:?}\n", self.0.lights));
        output.push_str(&format!("    Kind: {:?}\n", self.0.kind));
        if let Some(v) = self.0.class {
            output.push_str(&format!("    Class: {:?}\n", v));
        }
        if let Some(v) = self.0.state {
            output.push_str(&format!("    AnyOn: {}\n", v.any_on));
            output.push_str(&format!("    AllOn: {}\n", v.all_on));
        }
        if let Some(v) = &self.0.model_id {
            output.push_str(&format!("    ModelId: {:?}\n", v));
        }
        if let Some(v) = &self.0.unique_id {
            output.push_str(&format!("    UniqueId: {:?}\n", v));
        }
        output.pop();
        write!(f, "{}", output)
    }
}

pub fn set(arg: subcommand::group::Set) {
    let bridge = util::get_bridge();
    let mut responses = Vec::new();
    let state_modifier = arg.to_state_modifier();
    if !state_modifier.is_empty() {
        responses.extend(match bridge.set_group_state(&arg.id, &state_modifier) {
            Ok(v) => v,
            Err(e) => exit!("Error occured while modifying the state of the lights", e),
        });
    }
    let attribute_modifier = arg.to_attribute_modifier();
    if !attribute_modifier.is_empty() {
        responses.extend(
            match bridge.set_group_attribute(&arg.id, &attribute_modifier) {
                Ok(v) => v,
                Err(e) => exit!("Error occured while modifying attributes of the lights", e),
            },
        );
    }
    for i in responses {
        println!("{}", i);
    }
}

pub fn get(arg: subcommand::group::Get) {
    let bridge = util::get_bridge();
    match arg.id {
        Some(v) => match bridge.get_group(&v) {
            Ok(v) => println!("{}", Group(v)),
            Err(e) => exit!("Failed to get group", e),
        },
        None => match bridge.get_all_groups() {
            Ok(v) => {
                for group in v {
                    println!("{}\n", Group(group));
                }
            }
            Err(e) => exit!("Failed to get groups", e),
        },
    };
}

pub fn create(arg: subcommand::group::Create) {
    match util::get_bridge().create_group(&arg.to_creator()) {
        Ok(v) => println!("Created group {}", v),
        Err(e) => exit!("Failed to create group", e),
    }
}

pub fn delete(arg: subcommand::group::Delete) {
    match util::get_bridge().delete_group(&arg.id) {
        Ok(_) => println!("Deleted group {}", arg.id),
        Err(e) => exit!("Failed to delete group", e),
    };
}
