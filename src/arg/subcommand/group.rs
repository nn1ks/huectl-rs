use crate::arg::value;
use huelib::Modifier;
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
        let mut modifier = huelib::group::StateModifier::new();
        if self.on {
            modifier = modifier.on(true);
        } else if self.off {
            modifier = modifier.on(false);
        }
        if let Some(v) = &self.brightness {
            modifier = modifier.brightness(v.modifier_type, v.value);
        }
        if let Some(v) = &self.hue {
            modifier = modifier.hue(v.modifier_type, v.value);
        }
        if let Some(v) = &self.saturation {
            modifier = modifier.saturation(v.modifier_type, v.value);
        }
        if let Some(v) = &self.color_space_coordinates {
            modifier = modifier.color_space_coordinates(v.modifier_type, v.value);
        }
        if let Some(v) = &self.color_temperature {
            modifier = modifier.color_temperature(v.modifier_type, v.value);
        }
        if let Some(v) = &self.alert {
            modifier = modifier.alert(v.value);
        }
        if let Some(v) = &self.effect {
            modifier = modifier.effect(v.value);
        }
        if let Some(v) = self.transition_time {
            modifier = modifier.transition_time(v);
        }
        modifier
    }

    pub fn to_attribute_modifier(&self) -> huelib::group::AttributeModifier {
        let mut modifier = huelib::group::AttributeModifier::new();
        if let Some(v) = &self.name {
            modifier = modifier.name(v);
        }
        if let Some(v) = &self.lights {
            modifier = modifier.lights(v.clone());
        }
        if let Some(v) = &self.class {
            modifier = modifier.class(v.value);
        }
        modifier
    }
}

/// Prints the state and attributes of a group
#[derive(Debug, StructOpt)]
pub struct Get {
    /// Identifier of the group, if omitted all groups are selected
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
