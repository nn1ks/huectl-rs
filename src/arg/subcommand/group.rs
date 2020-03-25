use crate::{arg::value, command};
use structopt::StructOpt;

/// Modifies the state and attributes of a group
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
    #[structopt(long, short, name = "x,y", allow_hyphen_values = true)]
    color_space_coordinates: Option<value::ColorSpaceCoordinates>,
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
    pub fn to_state_modifier(&self) -> huelib::group::StateModifier {
        let mut state_modifier = huelib::group::StateModifier::new();
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
            state_modifier = state_modifier.color_space_coordinates(v.modifier_type, v.value);
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

    pub fn to_attribute_modifier(&self) -> huelib::group::AttributeModifier {
        let mut attribute_modifier = huelib::group::AttributeModifier::new();
        if let Some(v) = &self.name {
            attribute_modifier = attribute_modifier.name(&v);
        }
        if let Some(v) = &self.lights {
            attribute_modifier =
                attribute_modifier.lights(&v.iter().map(AsRef::as_ref).collect::<Vec<&str>>());
        }
        if let Some(v) = &self.class {
            attribute_modifier = attribute_modifier.class(v.value);
        }
        attribute_modifier
    }
}

/// Prints the state and attributes of a group
#[derive(Debug, StructOpt)]
pub struct Get {
    /// Identifier of the group, if ommited all groups are selected
    pub id: Option<String>,
}

/// Creates a group
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
    pub fn to_creator(&self) -> huelib::group::Creator {
        huelib::group::Creator {
            name: self.name.clone(),
            lights: self.lights.clone(),
            kind: match &self.kind {
                Some(v) => Some(v.value),
                None => None,
            },
            class: match &self.class {
                Some(v) => Some(v.value),
                None => None,
            },
        }
    }
}

/// Deletes a group
#[derive(Debug, StructOpt)]
pub struct Delete {
    /// Identifier of the group
    pub id: String,
}
