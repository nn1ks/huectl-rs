use crate::{arg::value, command};
use structopt::StructOpt;

/// Modifies the state and attributes of a light
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
    #[structopt(long, short, name = "x,y", allow_hyphen_values = true)]
    color_space_coordinates: Option<value::ColorSpaceCoordinates>,
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
    pub fn to_modifier(&self) -> command::light::Modifier {
        let mut state_modifier = huelib::light::StateModifier::new();
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
        let mut attribute_modifier = huelib::light::AttributeModifier::new();
        if let Some(v) = &self.name {
            attribute_modifier = attribute_modifier.name(&v);
        }
        command::light::Modifier {
            state: state_modifier,
            attribute: attribute_modifier,
        }
    }
}

/// Prints the state and attributes of a light
#[derive(Debug, StructOpt)]
pub struct Get {
    /// Identifier of the light, if ommited all lights are selected
    pub id: Option<String>,
    /// Prints the output in JSON format
    #[structopt(long, short)]
    pub json: bool,
}

/// Searches for new lights
#[derive(Debug, StructOpt)]
pub struct Search {
    /// Prints the lights that were discovered by the last search
    #[structopt(long, short)]
    pub get: bool,
}

/// Deletes a light
#[derive(Debug, StructOpt)]
pub struct Delete {
    /// Identifier of the light
    pub id: String,
}
