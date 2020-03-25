use crate::{arg::subcommand, util};
use huelib::Modifier;
use std::fmt;

struct Light(huelib::Light);

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("Light {}:\n", self.0.id));
        output.push_str(&format!("    Name: {}\n", self.0.name));
        output.push_str(&format!("    Kind: {}\n", self.0.kind));
        if let Some(v) = self.0.state.brightness {
            output.push_str(&format!("    Brightness: {}\n", v));
        }
        if let Some(v) = self.0.state.hue {
            output.push_str(&format!("    Hue: {}\n", v));
        }
        if let Some(v) = self.0.state.saturation {
            output.push_str(&format!("    Saturation: {}\n", v));
        }
        if let Some(v) = self.0.state.color_space_coordinates {
            output.push_str(&format!(
                "    ColorSpaceCoordinates: {}\n",
                format!("{},{}", v.0, v.1)
            ));
        }
        if let Some(v) = self.0.state.color_temperature {
            output.push_str(&format!("    ColorTemperature: {}\n", v));
        }
        if let Some(v) = self.0.state.alert {
            output.push_str(&format!("    Alert: {:?}\n", v));
        }
        if let Some(v) = self.0.state.effect {
            output.push_str(&format!("    Effect: {:?}\n", v));
        }
        if let Some(v) = self.0.state.color_mode {
            output.push_str(&format!("    ColorMode: {:?}\n", v));
        }
        output.push_str(&format!("    Reachable: {}\n", self.0.state.reachable));
        output.push_str(&format!("    ModelId: {}\n", self.0.model_id));
        if let Some(v) = &self.0.product_id {
            output.push_str(&format!("    ProductId: {}\n", v));
        }
        if let Some(v) = &self.0.product_name {
            output.push_str(&format!("    ProductName: {}\n", v));
        }
        if let Some(v) = &self.0.manufacturer_name {
            output.push_str(&format!("    ManufacturerName: {}\n", v));
        }
        output.push_str(&format!(
            "    SoftwareVersion: {}\n",
            self.0.software_version
        ));
        output.push_str(&format!(
            "    SoftwareUpdateState: {:?}\n",
            self.0.software_update.state
        ));
        if let Some(v) = &self.0.software_update.last_install {
            output.push_str(&format!("    SoftwareUpdateLastInstall: {}\n", v));
        }
        output.pop();
        write!(f, "{}", output)
    }
}

pub fn set(arg: subcommand::light::Set) {
    let bridge = util::get_bridge();
    let mut responses = Vec::new();
    let state_modifier = arg.to_state_modifier();
    if !state_modifier.is_empty() {
        responses.extend(match bridge.set_light_state(&arg.id, &state_modifier) {
            Ok(v) => v,
            Err(e) => util::print_err("Error occured while modifying the state of the light", e),
        });
    }
    let attribute_modifier = arg.to_attribute_modifier();
    if !attribute_modifier.is_empty() {
        responses.extend(
            match bridge.set_light_attribute(&arg.id, &attribute_modifier) {
                Ok(v) => v,
                Err(e) => {
                    util::print_err("Error occured while modifying attributes of the light", e)
                }
            },
        );
    }
    for i in responses {
        println!("{}", i);
    }
}

pub fn get(arg: subcommand::light::Get) {
    let bridge = util::get_bridge();
    match arg.id {
        Some(v) => match bridge.get_light(&v) {
            Ok(v) => println!("{}", Light(v)),
            Err(e) => util::print_err("Failed to get light", e),
        },
        None => match bridge.get_all_lights() {
            Ok(v) => {
                for light in v {
                    println!("{}\n", Light(light));
                }
            }
            Err(e) => util::print_err("Failed to get lights", e),
        },
    };
}

pub fn search(arg: subcommand::light::Search) {
    let bridge = util::get_bridge();
    if arg.get {
        match bridge.get_new_lights() {
            Ok(v) => {
                use huelib::light::LastScan;
                match v.last_scan {
                    LastScan::DateTime(v) => println!("Last scan: {}", v),
                    LastScan::Active => {
                        println!("The bridge is currently searching for new lights")
                    }
                    LastScan::None => (),
                };
                if v.lights.is_empty() {
                    println!("No lights were discovered");
                } else {
                    println!("Discovered lights:");
                    for i in v.lights {
                        println!("{:?}", i);
                    }
                }
            }
            Err(e) => util::print_err("Failed to get new lights", e),
        };
    } else {
        match bridge.search_new_lights(None) {
            Ok(_) => println!("Searching for new lights"),
            Err(e) => util::print_err("Failed to search for new lights", e),
        };
    }
}

pub fn delete(arg: subcommand::light::Delete) {
    match util::get_bridge().delete_light(&arg.id) {
        Ok(_) => println!("Deleted light {}", arg.id),
        Err(e) => util::print_err("Failed to delete light", e),
    };
}
