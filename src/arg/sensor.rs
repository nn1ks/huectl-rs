use crate::{output::Sensor as OutputSensor, output::Scan as OutputScan, util};
use huelib::resource::{sensor, Modifier};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Arg {
    /// Modifies the state and attributes of a sensor
    Set(Set),
    /// Prints the state and attributes of a sensor
    Get(Get),
    /// Searches for new sensors
    Search(Search),
    /// Deletes a sensor
    Delete(Delete),
}

#[derive(Debug, StructOpt)]
pub struct Set {
    /// Identifier of the sensor
    pub id: String,
    /// Turns the sensor on
    #[structopt(long)]
    on: bool,
    /// Turns the sensor off
    #[structopt(long)]
    off: bool,
    /// Sets the name of the sensor
    #[structopt(long, short)]
    name: Option<String>,
    /// Enables presence
    #[structopt(long, short)]
    presence: bool,
    /// Disables presence
    #[structopt(long, short = "P")]
    no_presence: bool,
}

impl Set {
    pub fn to_state_modifier(&self) -> sensor::StateModifier {
        let mut modifier = sensor::StateModifier::new();
        if self.presence {
            modifier = modifier.presence(true);
        } else if self.no_presence {
            modifier = modifier.presence(false);
        }
        modifier
    }

    pub fn to_attribute_modifier(&self) -> sensor::AttributeModifier {
        let mut modifier = sensor::AttributeModifier::new();
        if let Some(v) = &self.name {
            modifier = modifier.name(v);
        }
        modifier
    }

    pub fn to_config_modifier(&self) -> sensor::ConfigModifier {
        let mut modifier = sensor::ConfigModifier::new();
        if self.on {
            modifier = modifier.on(true);
        } else if self.off {
            modifier = modifier.on(false);
        }
        modifier
    }
}

pub fn set(arg: Set) {
    let bridge = util::get_bridge();
    let mut responses = Vec::new();
    let state_modifier = arg.to_state_modifier();
    if !state_modifier.is_empty() {
        responses.extend(match bridge.set_sensor_state(&arg.id, &state_modifier) {
            Ok(v) => v,
            Err(e) => exit!("Error occured while modifying the state of the sensor", e),
        });
    }
    let attribute_modifier = arg.to_attribute_modifier();
    if !attribute_modifier.is_empty() {
        responses.extend(
            match bridge.set_sensor_attribute(&arg.id, &attribute_modifier) {
                Ok(v) => v,
                Err(e) => exit!("Error occured while modifying attributes of the sensor", e),
            },
        );
    }
    let config_modifier = arg.to_config_modifier();
    if !config_modifier.is_empty() {
        responses.extend(
            match bridge.set_sensor_config(&arg.id, &config_modifier) {
                Ok(v) => v,
                Err(e) => exit!("Error occured while modifying the config of the sensor", e),
            },
        );
    }
    for i in responses {
        println!("{}", i);
    }
}

#[derive(Debug, StructOpt)]
pub struct Get {
    /// Identifier of the sensor, if omitted all sensors are selected
    pub id: Option<String>,
}

pub fn get(arg: Get) {
    let bridge = util::get_bridge();
    match arg.id {
        Some(v) => match bridge.get_sensor(&v) {
            Ok(v) => println!(
                "{}",
                serde_json::to_string_pretty(&OutputSensor::from(v)).unwrap()
            ),
            Err(e) => exit!("Failed to get sensor", e),
        },
        None => match bridge.get_all_sensors() {
            Ok(v) => {
                let sensors: Vec<OutputSensor> = v.into_iter().map(OutputSensor::from).collect();
                println!("{}", serde_json::to_string_pretty(&sensors).unwrap());
            }
            Err(e) => exit!("Failed to get sensors", e),
        },
    };
}

#[derive(Debug, StructOpt)]
pub struct Search {
    /// Prints the sensors that were discovered by the last search
    #[structopt(long, short)]
    pub get: bool,
}

pub fn search(arg: Search) {
    let bridge = util::get_bridge();
    if arg.get {
        match bridge.get_new_sensors() {
            Ok(v) => println!(
                "{}",
                serde_json::to_string_pretty(&OutputScan::from(v)).unwrap()
            ),
            Err(e) => exit!("Failed to get new sensors", e),
        };
    } else {
        match bridge.search_new_sensors(None) {
            Ok(_) => println!("Searching for new sensors..."),
            Err(e) => exit!("Failed to search for new sensors", e),
        };
    }
}

#[derive(Debug, StructOpt)]
pub struct Delete {
    /// Identifier of the sensor
    pub id: String,
}

pub fn delete(arg: Delete) {
    match util::get_bridge().delete_sensor(&arg.id) {
        Ok(_) => println!("Deleted sensor {}", arg.id),
        Err(e) => exit!("Failed to delete sensor", e),
    };
}
