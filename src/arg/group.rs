use crate::{arg::value, output::Group as OutputGroup, util};
use huelib::resource::{group, Modifier};
use huelib::Color;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Arg {
    /// Modifies the state and attributes of a group
    Set(Set),
    /// Prints the state and attributes of a group
    Get(Get),
    /// Creates a group
    Create(Create),
    /// Deletes a group
    Delete(Delete),
}

#[derive(Debug, StructOpt)]
pub struct Set {
    /// Identifier of the group
    pub id: String,
    /// Turns the lights on
    #[structopt(long)]
    on: bool,
    /// Turns the lights off
    #[structopt(long)]
    off: bool,
    /// Sets the brightness of the lights in percentage
    #[structopt(long, short, allow_hyphen_values = true)]
    brightness: Option<value::Brightness>,
    /// Sets the hue of the lights
    #[structopt(long, allow_hyphen_values = true)]
    hue: Option<value::Hue>,
    /// Sets the saturation of the lights in percentage
    #[structopt(long, short, allow_hyphen_values = true)]
    saturation: Option<value::Saturation>,
    /// Sets the color temperature of the lights
    #[structopt(long, short = "t", allow_hyphen_values = true)]
    color_temperature: Option<value::ColorTemperature>,
    /// Sets the x and y coordinates in the color space of the lights
    #[structopt(long, short, name = "coordinate", min_values = 2, max_values = 2)]
    color_space_coordinates: Option<Vec<f32>>,
    /// Sets the color of the lights with red, green, and blue values
    #[structopt(long, short = "r", min_values = 3, max_values = 3)]
    color_rgb: Option<Vec<u8>>,
    /// Sets the color of the lights with a hex value
    #[structopt(long, short = "x")]
    color_hex: Option<value::ColorHex>,
    /// Sets the alert effect of the lights
    #[structopt(long, short, case_insensitive = true, possible_values = value::Alert::variants())]
    alert: Option<value::Alert>,
    /// Sets the dynamic effect of the lights
    #[structopt(long, short, case_insensitive = true, possible_values = value::Effect::variants())]
    effect: Option<value::Effect>,
    /// Sets the transition time of the lights
    #[structopt(long)]
    transition_time: Option<u16>,
    /// Renames the group
    #[structopt(long, short)]
    name: Option<String>,
    /// Sets the lights that are in the group
    #[structopt(long, short)]
    lights: Option<Vec<String>>,
    /// Sets the class of the group
    #[structopt(long, case_insensitive = true, possible_values = value::GroupClass::variants())]
    class: Option<value::GroupClass>,
}

impl Set {
    pub fn to_state_modifier(&self) -> group::StateModifier {
        let mut modifier = group::StateModifier::new();
        if self.on {
            modifier = modifier.on(true);
        } else if self.off {
            modifier = modifier.on(false);
        }
        if let Some(v) = &self.brightness {
            modifier = modifier.brightness(v.0, v.1);
        }
        if let Some(v) = &self.hue {
            modifier = modifier.hue(v.0, v.1);
        }
        if let Some(v) = &self.saturation {
            modifier = modifier.saturation(v.0, v.1);
        }
        if let Some(v) = &self.color_space_coordinates {
            modifier = modifier.color(Color::from_space_coordinates(v[0], v[1]));
        }
        if let Some(v) = &self.color_rgb {
            modifier = modifier.color(Color::from_rgb(v[0], v[1], v[2]));
        }
        if let Some(v) = &self.color_hex {
            modifier = modifier.color(v.0);
        }
        if let Some(v) = &self.color_temperature {
            modifier = modifier.color_temperature(v.0, v.1);
        }
        if let Some(v) = &self.alert {
            modifier = modifier.alert(v.0);
        }
        if let Some(v) = &self.effect {
            modifier = modifier.effect(v.0);
        }
        if let Some(v) = self.transition_time {
            modifier = modifier.transition_time(v);
        }
        modifier
    }

    pub fn to_attribute_modifier(&self) -> group::AttributeModifier {
        let mut modifier = group::AttributeModifier::new();
        if let Some(v) = &self.name {
            modifier = modifier.name(v);
        }
        if let Some(v) = &self.lights {
            modifier = modifier.lights(v.clone());
        }
        if let Some(v) = &self.class {
            modifier = modifier.class(v.0);
        }
        modifier
    }
}

pub fn set(arg: Set) {
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

#[derive(Debug, StructOpt)]
pub struct Get {
    /// Identifier of the group, if omitted all groups are selected
    pub id: Option<String>,
}

pub fn get(arg: Get) {
    let bridge = util::get_bridge();
    match arg.id {
        Some(v) => match bridge.get_group(&v) {
            Ok(v) => println!(
                "{}",
                serde_json::to_string_pretty(&OutputGroup::from(v)).unwrap()
            ),
            Err(e) => exit!("Failed to get group", e),
        },
        None => match bridge.get_all_groups() {
            Ok(v) => {
                let groups: Vec<OutputGroup> = v.into_iter().map(OutputGroup::from).collect();
                println!("{}", serde_json::to_string_pretty(&groups).unwrap());
            }
            Err(e) => exit!("Failed to get groups", e),
        },
    };
}

#[derive(Debug, StructOpt)]
pub struct Create {
    /// The name of the new group
    name: String,
    /// Sets the indentifiers of the lights that will be in this group
    #[structopt(long, short)]
    lights: Vec<String>,
    /// Sets the type of the group
    #[structopt(long, short, case_insensitive = true, possible_values = value::GroupTypeCreator::variants())]
    kind: Option<value::GroupTypeCreator>,
    /// Sets the class of the group
    #[structopt(long, case_insensitive = true, possible_values = value::GroupClass::variants())]
    class: Option<value::GroupClass>,
}

impl Create {
    pub fn to_creator(&self) -> group::Creator {
        let mut creator = group::Creator::new(&self.name, self.lights.clone());
        if let Some(v) = &self.kind {
            creator = creator.kind(v.0);
        }
        if let Some(v) = &self.class {
            creator = creator.class(v.0);
        }
        creator
    }
}

pub fn create(arg: Create) {
    match util::get_bridge().create_group(&arg.to_creator()) {
        Ok(v) => println!("Created group {}", v),
        Err(e) => exit!("Failed to create group", e),
    }
}

#[derive(Debug, StructOpt)]
pub struct Delete {
    /// Identifier of the group
    pub id: String,
}

pub fn delete(arg: Delete) {
    match util::get_bridge().delete_group(&arg.id) {
        Ok(_) => println!("Deleted group {}", arg.id),
        Err(e) => exit!("Failed to delete group", e),
    };
}
