use crate::arg;
use crate::command;
use std::net::IpAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    Discover(Discover),
    Register(Register),
    SetConfig(SetConfig),
    GetConfig(GetConfig),
    SetLight(SetLight),
    GetLight(GetLight),
    SearchLight(SearchLight),
    DeleteLight(DeleteLight),
    CreateGroup(CreateGroup),
    SetGroup(SetGroup),
    GetGroup(GetGroup),
    DeleteGroup(DeleteGroup),
    CreateScene(CreateScene),
    SetScene(SetScene),
    GetScene(GetScene),
}

/// Discovers bridges in the local network
#[derive(Debug, StructOpt)]
pub struct Discover {}

/// Registers a new user on a bridge
#[derive(Debug, StructOpt)]
pub struct Register {
    /// IP address of the bridge
    pub ip_address: IpAddr,
    /// Sets environment variables for the current session
    #[structopt(long, short)]
    pub set_env: bool,
}

/// Modifies attributes of the bridge configuration
#[derive(Debug, StructOpt)]
pub struct SetConfig {
    /// Sets the name of the bridge
    #[structopt(long, short)]
    pub name: Option<String>,
    /// Sets the ip address of the bridge
    #[structopt(long, short)]
    pub ip_address: Option<IpAddr>,
    /// Sets the network mask of the bridge
    #[structopt(long)]
    pub netmask: Option<String>,
    /// Sets the gateway ip address of the bridge
    #[structopt(long, short)]
    pub gateway: Option<IpAddr>,
    /// Activates DHCP for the bridge
    #[structopt(long, short)]
    pub dhcp: bool,
    /// Deactivates DHCP for the bridge
    #[structopt(long, short = "D")]
    pub no_dhcp: bool,
    /// Sets the proxy port of the bridge
    #[structopt(long)]
    pub proxy_port: Option<u16>,
    /// Sets the proxy address of the bridge
    #[structopt(long)]
    pub proxy_address: Option<IpAddr>,
    /// Deactives the proxy on the bridge
    #[structopt(long)]
    pub no_proxy: bool,
    /// Adds the closest lamp to the ZigBee network
    #[structopt(long, short)]
    pub touchlink: bool,
    /// Sets the ZigBee channel
    #[structopt(long, short)]
    pub zigbee_channel: Option<u8>,
    /// Sets the current time of the bridge
    #[structopt(long)]
    pub current_time: Option<String>,
    /// Sets the timezone of the bridge
    #[structopt(long)]
    pub timezone: Option<String>,
}

impl SetConfig {
    pub fn to_modifier(&self) -> huelib::config::Modifier {
        let mut modifier = huelib::config::Modifier::new();
        if let Some(v) = &self.name {
            modifier = modifier.name(v);
        }
        if let Some(v) = self.ip_address {
            modifier = modifier.ip_address(v);
        }
        if let Some(v) = &self.netmask {
            modifier = modifier.netmask(v);
        }
        if let Some(v) = self.gateway {
            modifier = modifier.gateway(v);
        }
        if self.dhcp {
            modifier = modifier.dhcp(true);
        } else if self.no_dhcp {
            modifier = modifier.dhcp(false);
        }
        if let Some(v) = self.proxy_port {
            modifier = modifier.proxy_port(v);
        }
        if let Some(v) = self.proxy_address {
            modifier = modifier.proxy_address(v);
        }
        if self.no_proxy {
            modifier = modifier.no_proxy();
        }
        if self.touchlink {
            modifier = modifier.touchlink();
        }
        if let Some(v) = self.zigbee_channel {
            modifier = modifier.zigbee_channel(v);
        }
        if let Some(v) = &self.current_time {
            modifier = modifier.current_time(v);
        }
        if let Some(v) = &self.timezone {
            modifier = modifier.timezone(v);
        }
        modifier
    }
}

/// Prints the bridge configuration
#[derive(Debug, StructOpt)]
pub struct GetConfig {
    /// Prints the output in JSON format
    #[structopt(long, short)]
    pub json: bool,
}

/// Modifies the state and attributes of a light
#[derive(Debug, StructOpt)]
pub struct SetLight {
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
    brightness: Option<arg::value::Brightness>,
    /// Sets the hue of the light
    #[structopt(long, allow_hyphen_values = true)]
    hue: Option<arg::value::Hue>,
    /// Sets the saturation of the light in percentage
    #[structopt(long, short, allow_hyphen_values = true)]
    saturation: Option<arg::value::Saturation>,
    /// Sets the color temperature of the light
    #[structopt(long, short = "t", allow_hyphen_values = true)]
    color_temperature: Option<arg::value::ColorTemperature>,
    /// Sets the x and y coordinates in the color space of the light
    #[structopt(long, short, name = "x,y", allow_hyphen_values = true)]
    color_space_coordinates: Option<arg::value::ColorSpaceCoordinates>,
    /// Sets the alert effect of the light
    #[structopt(long, short, case_insensitive = true, possible_values = arg::value::Alert::variants())]
    alert: Option<arg::value::Alert>,
    /// Sets the dynamic effect of the light
    #[structopt(long, short, case_insensitive = true, possible_values = arg::value::Effect::variants())]
    effect: Option<arg::value::Effect>,
    /// Sets the transition time of the light
    #[structopt(long)]
    transition_time: Option<u16>,
    /// Renames the light
    #[structopt(long, short)]
    name: Option<String>,
}

impl SetLight {
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
pub struct GetLight {
    /// Identifier of the light, if ommited all lights are selected
    pub id: Option<String>,
    /// Prints the output in JSON format
    #[structopt(long, short)]
    pub json: bool,
}

/// Searches for new lights
#[derive(Debug, StructOpt)]
pub struct SearchLight {
    /// Prints the lights that were discovered by the last search
    #[structopt(long, short)]
    pub get: bool,
}

/// Deletes a light
#[derive(Debug, StructOpt)]
pub struct DeleteLight {
    /// Identifier of the light
    pub id: String,
}

/// Creates a group
#[derive(Debug, StructOpt)]
pub struct CreateGroup {
    /// The name of the new group
    name: String,
    /// Sets the indentifiers of the lights that will be in this group
    #[structopt(long, short)]
    lights: Vec<String>,
    /// Sets the type of the group
    #[structopt(long, short, case_insensitive = true, possible_values = arg::value::GroupTypeCreator::variants())]
    kind: Option<arg::value::GroupTypeCreator>,
    /// Sets the class of the group
    #[structopt(long, case_insensitive = true, possible_values = arg::value::GroupClass::variants())]
    class: Option<arg::value::GroupClass>,
}

impl CreateGroup {
    pub fn to_creator(self) -> huelib::group::Creator {
        huelib::group::Creator {
            name: self.name,
            lights: self.lights,
            kind: self.kind.map(|v| v.value),
            class: self.class.map(|v| v.value),
        }
    }
}

/// Modifies the state and attributes of a group
#[derive(Debug, StructOpt)]
pub struct SetGroup {
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
    brightness: Option<arg::value::Brightness>,
    /// Sets the hue of the lights
    #[structopt(long, allow_hyphen_values = true)]
    hue: Option<arg::value::Hue>,
    /// Sets the saturation of the lights in percentage
    #[structopt(long, short, allow_hyphen_values = true)]
    saturation: Option<arg::value::Saturation>,
    /// Sets the color temperature of the lights
    #[structopt(long, short = "t", allow_hyphen_values = true)]
    color_temperature: Option<arg::value::ColorTemperature>,
    /// Sets the x and y coordinates in the color space of the lights
    #[structopt(long, short, name = "x,y", allow_hyphen_values = true)]
    color_space_coordinates: Option<arg::value::ColorSpaceCoordinates>,
    /// Sets the alert effect of the lights
    #[structopt(long, short, case_insensitive = true, possible_values = arg::value::Alert::variants())]
    alert: Option<arg::value::Alert>,
    /// Sets the dynamic effect of the lights
    #[structopt(long, short, case_insensitive = true, possible_values = arg::value::Effect::variants())]
    effect: Option<arg::value::Effect>,
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
    #[structopt(long, case_insensitive = true, possible_values = arg::value::GroupClass::variants())]
    class: Option<arg::value::GroupClass>,
}

impl SetGroup {
    pub fn to_modifier(&self) -> command::group::Modifier {
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
        command::group::Modifier {
            state: state_modifier,
            attribute: attribute_modifier,
        }
    }
}

/// Prints the state and attributes of a group
#[derive(Debug, StructOpt)]
pub struct GetGroup {
    /// Identifier of the group, if ommited all groups are selected
    pub id: Option<String>,
    /// Prints the output in JSON format
    #[structopt(long, short)]
    pub json: bool,
}

/// Deletes a group
#[derive(Debug, StructOpt)]
pub struct DeleteGroup {
    /// Identifier of the group
    pub id: String,
}

/// Creates a scene
#[derive(Debug, StructOpt)]
pub struct CreateScene {
    /// The name of the new scene
    name: String,
    /// Sets the identifiers of the lights that will be in this scene
    #[structopt(long, short)]
    lights: Vec<String>,
    /// Sets the type of the scene
    #[structopt(long, short, case_insensitive = true, possible_values = arg::value::SceneType::variants())]
    kind: Option<arg::value::SceneType>,
    /// Sets the app version of the scene
    #[structopt(long)]
    app_version: Option<i8>,
    /// Sets the app data of the scene
    #[structopt(long)]
    app_data: Option<String>,
}

impl CreateScene {
    pub fn to_creator(self) -> huelib::scene::Creator {
        let mut creator = huelib::scene::Creator::new(&self.name, self.lights);
        if let Some(v) = self.kind {
            creator = creator.kind(v.value);
        }
        let mut app_data = huelib::scene::AppData::new();
        if let Some(v) = self.app_version {
            app_data = app_data.version(v);
        }
        if let Some(v) = self.app_data {
            app_data = app_data.data(v);
        }
        if app_data.version != None || app_data.data != None {
            creator = creator.app_data(app_data);
        }
        creator
    }
}

/// Modifies the state and attributes of a scene
#[derive(Debug, StructOpt)]
pub struct SetScene {
    /// Identifier of the scene
    pub id: String,
    /// Renames the scene
    #[structopt(long, short)]
    name: Option<String>,
    /// Sets the identifiers of the lights in this scene
    #[structopt(long, short)]
    lights: Option<Vec<String>>,
    /// Stores the light state
    #[structopt(long, short)]
    store_light_state: bool,
    /// Does not store the light state
    #[structopt(long, short = "S")]
    no_store_light_state: bool,
    // TODO: Add options for changing light state
}

impl SetScene {
    pub fn to_modifier(&self) -> huelib::scene::Modifier {
        let mut modifier = huelib::scene::Modifier::new();
        if let Some(v) = &self.name {
            modifier = modifier.name(v);
        }
        if let Some(v) = &self.lights {
            modifier = modifier.lights(v.to_vec());
        }
        if self.store_light_state {
            modifier = modifier.store_light_state(true);
        }
        if self.no_store_light_state {
            modifier = modifier.store_light_state(false);
        }
        modifier
    }
}

/// Prints the state and attributes of a scene
#[derive(Debug, StructOpt)]
pub struct GetScene {
    /// Identifier of the scene, if ommited all scenes are selected
    pub id: Option<String>,
    /// Prints the output in JSON format
    #[structopt(long, short)]
    pub json: bool,
}
