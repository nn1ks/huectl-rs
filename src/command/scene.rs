use crate::{arg::subcommand, util};
use std::fmt;

struct Scene(huelib::Scene);

impl fmt::Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("Scene {}:\n", self.0.id));
        output.push_str(&format!("    Name: {}\n", self.0.name));
        output.push_str(&format!("    Kind: {:?}\n", self.0.kind));
        if let Some(v) = &self.0.group {
            output.push_str(&format!("    Group: {}\n", v));
        }
        if let Some(v) = &self.0.lights {
            output.push_str(&format!("    Lights: {:?}\n", v));
        }
        if let Some(v) = &self.0.owner {
            output.push_str(&format!("    Owner: {}\n", v));
        }
        output.push_str(&format!("    Recycle: {}\n", self.0.recycle));
        output.push_str(&format!("    Locked: {}\n", self.0.locked));
        if let Some(v) = self.0.app_data.version {
            output.push_str(&format!("    AppVersion: {}\n", v));
        }
        if let Some(v) = &self.0.app_data.data {
            output.push_str(&format!("    AppData: {}\n", v));
        }
        if let Some(v) = &self.0.picture {
            if !v.is_empty() {
                output.push_str(&format!("    Picture: {}\n", v));
            }
        }
        if let Some(v) = self.0.last_update {
            output.push_str(&format!("    LastUpdate: {}\n", v));
        }
        output.push_str(&format!("    Version: {:?}\n", self.0.version));
        output.pop();
        write!(f, "{}", output)
    }
}

pub fn set(arg: subcommand::scene::Set) {
    let responses = match util::get_bridge().set_scene(&arg.id, &arg.to_modifier()) {
        Ok(v) => v,
        Err(e) => util::print_err("Failed to set scene", e),
    };
    for i in responses {
        println!("{}", i);
    }
}

pub fn get(arg: subcommand::scene::Get) {
    let bridge = util::get_bridge();
    match arg.id {
        Some(v) => match bridge.get_scene(&v) {
            Ok(v) => println!("{}", Scene(v)),
            Err(e) => util::print_err("Failed to get scene", e),
        },
        None => match bridge.get_all_scenes() {
            Ok(v) => {
                for scene in v {
                    println!("{}\n", Scene(scene));
                }
            }
            Err(e) => util::print_err("Failed to get scenes", e),
        },
    };
}

pub fn create(arg: subcommand::scene::Create) {
    match util::get_bridge().create_scene(&arg.to_creator()) {
        Ok(v) => println!("Created scene {}", v),
        Err(e) => util::print_err("Failed to create scene", e),
    };
}

pub fn delete(arg: subcommand::scene::Delete) {
    match util::get_bridge().delete_scene(&arg.id) {
        Ok(_) => println!("Deleted scene {}", arg.id),
        Err(e) => util::print_err("Failed to delete scene", e),
    };
}
