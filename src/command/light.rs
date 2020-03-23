use crate::arg::subcommand;
use crate::util;
use std::fmt;

pub struct Modifier {
    pub state: huelib::light::StateModifier,
    pub attribute: huelib::light::AttributeModifier,
}

struct Light {
    value: huelib::Light,
}

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("Light {}:\n", self.value.id));
        output.push_str(&format!("    Name: {}\n", self.value.name));
        output.push_str(&format!("    Kind: {}\n", self.value.kind));
        if let Some(v) = self.value.state.brightness {
            output.push_str(&format!("    Brightness: {}\n", v));
        }
        if let Some(v) = self.value.state.hue {
            output.push_str(&format!("    Hue: {}\n", v));
        }
        if let Some(v) = self.value.state.saturation {
            output.push_str(&format!("    Saturation: {}\n", v));
        }
        if let Some(v) = self.value.state.color_space_coordinates {
            output.push_str(&format!(
                "    ColorSpaceCoordinates: {}\n",
                format!("{},{}", v.0, v.1)
            ));
        }
        if let Some(v) = self.value.state.color_temperature {
            output.push_str(&format!("    ColorTemperature: {}\n", v));
        }
        if let Some(v) = self.value.state.alert {
            output.push_str(&format!("    Alert: {:?}\n", v));
        }
        if let Some(v) = self.value.state.effect {
            output.push_str(&format!("    Effect: {:?}\n", v));
        }
        if let Some(v) = self.value.state.color_mode {
            output.push_str(&format!("    ColorMode: {:?}\n", v));
        }
        output.push_str(&format!("    Reachable: {}\n", self.value.state.reachable));
        output.push_str(&format!("    ModelId: {}\n", self.value.model_id));
        if let Some(v) = &self.value.product_id {
            output.push_str(&format!("    ProductId: {}\n", v));
        }
        if let Some(v) = &self.value.product_name {
            output.push_str(&format!("    ProductName: {}\n", v));
        }
        if let Some(v) = &self.value.manufacturer_name {
            output.push_str(&format!("    ManufacturerName: {}\n", v));
        }
        output.push_str(&format!(
            "    SoftwareVersion: {}\n",
            self.value.software_version
        ));
        output.push_str(&format!(
            "    SoftwareUpdateState: {:?}\n",
            self.value.software_update.state
        ));
        if let Some(v) = &self.value.software_update.last_install {
            output.push_str(&format!("    SoftwareUpdateLastInstall: {}\n", v));
        }
        output.pop();
        write!(f, "{}", output)
    }
}

impl std::convert::From<huelib::Light> for Light {
    fn from(value: huelib::Light) -> Self {
        Self { value }
    }
}

pub fn set(arg: subcommand::SetLight) {
    let bridge = util::get_bridge();
    let modifier = arg.to_modifier();
    let mut responses = Vec::new();
    if !modifier.state.is_empty() {
        responses.extend(match bridge.set_light_state(&arg.id, &modifier.state) {
            Ok(v) => v,
            Err(e) => util::print_err("Error occured while modifying the state of the light", e),
        });
    }
    if !modifier.attribute.is_empty() {
        responses.extend(
            match bridge.set_light_attribute(&arg.id, &modifier.attribute) {
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

pub fn get(arg: subcommand::GetLight) {
    let bridge = util::get_bridge();
    match arg.id {
        Some(v) => match bridge.get_light(&v) {
            Ok(v) => {
                if arg.json {
                    match serde_json::to_string_pretty(&v) {
                        Ok(v) => println!("{}", v),
                        Err(e) => util::print_err("Failed to serialize data", e),
                    };
                } else {
                    println!("{}", Light::from(v));
                }
            }
            Err(e) => util::print_err("Failed to get light", e),
        },
        None => match bridge.get_all_lights() {
            Ok(v) => {
                if arg.json {
                    match serde_json::to_string_pretty(&v) {
                        Ok(v) => println!("{}", v),
                        Err(e) => util::print_err("Failed to serialize data", e),
                    };
                } else {
                    for light in v {
                        println!("{}\n", Light::from(light));
                    }
                }
            }
            Err(e) => util::print_err("Failed to get lights", e),
        },
    };
}

pub fn search(arg: subcommand::SearchLight) {
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

pub fn delete(arg: subcommand::DeleteLight) {
    match util::get_bridge().delete_light(&arg.id) {
        Ok(_) => println!("Deleted light {}", arg.id),
        Err(e) => util::print_err("Failed to delete light", e),
    };
}
