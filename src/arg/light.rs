use crate::{arg::value, util};
use huelib::resource::{self, light, Modifier};
use huelib::Color;
use std::fmt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Arg {
    /// Modifies the state and attributes of a light
    Set(Set),
    /// Prints the state and attributes of a light
    Get(Get),
    /// Searches for new lights
    Search(Search),
    /// Deletes a light
    Delete(Delete),
}

#[derive(Debug, StructOpt)]
pub struct Set {
    /// Identifier of the light
    pub id: String,
    /// Turns the light on
    #[structopt(long)]
    on: bool,
    /// Turns the light off
    #[structopt(long)]
    off: bool,
    /// Sets the brightness of the light in percentage
    #[structopt(long, short, allow_hyphen_values = true)]
    brightness: Option<value::Brightness>,
    /// Sets the hue of the light
    #[structopt(long, allow_hyphen_values = true)]
    hue: Option<value::Hue>,
    /// Sets the saturation of the light in percentage
    #[structopt(long, short, allow_hyphen_values = true)]
    saturation: Option<value::Saturation>,
    /// Sets the color temperature of the light
    #[structopt(long, short = "t", allow_hyphen_values = true)]
    color_temperature: Option<value::ColorTemperature>,
    /// Sets the x and y coordinates in the color space of the light
    #[structopt(long, short, name = "coordinate", min_values = 2, max_values = 2)]
    color_space_coordinates: Option<Vec<f32>>,
    /// Sets the color of the light with red, green, and blue values
    #[structopt(long, short = "r", min_values = 3, max_values = 3)]
    color_rgb: Option<Vec<u8>>,
    /// Sets the color of the light with a hex value
    #[structopt(long, short = "x")]
    color_hex: Option<value::ColorHex>,
    /// Sets the alert effect of the light
    #[structopt(long, short, case_insensitive = true, possible_values = value::Alert::variants())]
    alert: Option<value::Alert>,
    /// Sets the dynamic effect of the light
    #[structopt(long, short, case_insensitive = true, possible_values = value::Effect::variants())]
    effect: Option<value::Effect>,
    /// Sets the transition time of the light
    #[structopt(long)]
    transition_time: Option<u16>,
    /// Renames the light
    #[structopt(long, short)]
    name: Option<String>,
}

impl Set {
    pub fn to_state_modifier(&self) -> light::StateModifier {
        let mut state_modifier = light::StateModifier::new();
        if self.on {
            state_modifier = state_modifier.on(true);
        } else if self.off {
            state_modifier = state_modifier.on(false);
        }
        if let Some(v) = &self.brightness {
            state_modifier = state_modifier.brightness(v.modifier_type, v.value);
        }
        if let Some(v) = &self.hue {
            state_modifier = state_modifier.hue(v.modifier_type, v.value);
        }
        if let Some(v) = &self.saturation {
            state_modifier = state_modifier.saturation(v.modifier_type, v.value);
        }
        if let Some(v) = &self.color_space_coordinates {
            state_modifier = state_modifier.color(Color::from_space_coordinates(v[0], v[1]));
        }
        if let Some(v) = &self.color_rgb {
            state_modifier = state_modifier.color(Color::from_rgb(v[0], v[1], v[2]));
        }
        if let Some(v) = &self.color_hex {
            state_modifier = state_modifier.color(v.value);
        }
        if let Some(v) = &self.color_temperature {
            state_modifier = state_modifier.color_temperature(v.modifier_type, v.value);
        }
        if let Some(v) = &self.alert {
            state_modifier = state_modifier.alert(v.value);
        }
        if let Some(v) = &self.effect {
            state_modifier = state_modifier.effect(v.value);
        }
        if let Some(v) = self.transition_time {
            state_modifier = state_modifier.transition_time(v);
        }
        state_modifier
    }

    pub fn to_attribute_modifier(&self) -> light::AttributeModifier {
        let mut attribute_modifier = light::AttributeModifier::new();
        if let Some(v) = &self.name {
            attribute_modifier = attribute_modifier.name(v);
        }
        attribute_modifier
    }
}

pub fn set(arg: Set) {
    let bridge = util::get_bridge();
    let mut responses = Vec::new();
    let state_modifier = arg.to_state_modifier();
    if !state_modifier.is_empty() {
        responses.extend(match bridge.set_light_state(&arg.id, &state_modifier) {
            Ok(v) => v,
            Err(e) => exit!("Error occured while modifying the state of the light", e),
        });
    }
    let attribute_modifier = arg.to_attribute_modifier();
    if !attribute_modifier.is_empty() {
        responses.extend(
            match bridge.set_light_attribute(&arg.id, &attribute_modifier) {
                Ok(v) => v,
                Err(e) => exit!("Error occured while modifying attributes of the light", e),
            },
        );
    }
    for i in responses {
        println!("{}", i);
    }
}

#[derive(Debug, StructOpt)]
pub struct Get {
    /// Identifier of the light, if omitted all lights are selected
    pub id: Option<String>,
}

pub fn get(arg: Get) {
    let bridge = util::get_bridge();
    match arg.id {
        Some(v) => match bridge.get_light(&v) {
            Ok(v) => println!("{}", LightDisplay(v)),
            Err(e) => exit!("Failed to get light", e),
        },
        None => match bridge.get_all_lights() {
            Ok(v) => {
                for light in v {
                    println!("{}\n", LightDisplay(light));
                }
            }
            Err(e) => exit!("Failed to get lights", e),
        },
    };
}

#[derive(Debug, StructOpt)]
pub struct Search {
    /// Prints the lights that were discovered by the last search
    #[structopt(long, short)]
    pub get: bool,
}

pub fn search(arg: Search) {
    let bridge = util::get_bridge();
    if arg.get {
        match bridge.get_new_lights() {
            Ok(v) => {
                use resource::LastScan;
                match v.last_scan {
                    LastScan::DateTime(v) => println!("Last scan: {}", v),
                    LastScan::Active => {
                        println!("The bridge is currently searching for new lights")
                    }
                    LastScan::None => (),
                };
                if v.resources.is_empty() {
                    println!("No lights were discovered");
                } else {
                    println!("Discovered lights:");
                    for i in v.resources {
                        println!("{:?}", i);
                    }
                }
            }
            Err(e) => exit!("Failed to get new lights", e),
        };
    } else {
        match bridge.search_new_lights(None) {
            Ok(_) => println!("Searching for new lights"),
            Err(e) => exit!("Failed to search for new lights", e),
        };
    }
}

#[derive(Debug, StructOpt)]
pub struct Delete {
    /// Identifier of the light
    pub id: String,
}

pub fn delete(arg: Delete) {
    match util::get_bridge().delete_light(&arg.id) {
        Ok(_) => println!("Deleted light {}", arg.id),
        Err(e) => exit!("Failed to delete light", e),
    };
}

struct LightDisplay(resource::Light);

impl fmt::Display for LightDisplay {
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
