use crate::{arg::value, util};
use huelib::resource::{self, scene, Modifier};
use std::fmt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Arg {
    /// Modifies the state and attributes of a scene
    Set(Set),
    /// Prints the state and attributes of a scene
    Get(Get),
    /// Creates a scene
    Create(Create),
    /// Deletes a scene
    Delete(Delete),
}

#[derive(Debug, StructOpt)]
pub struct Set {
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

impl Set {
    pub fn to_modifier(&self) -> scene::Modifier {
        let mut modifier = scene::Modifier::new();
        if let Some(v) = &self.name {
            modifier = modifier.name(v);
        }
        if let Some(v) = &self.lights {
            modifier = modifier.lights(v.clone());
        }
        if self.store_light_state {
            modifier = modifier.store_light_state(true);
        } else if self.no_store_light_state {
            modifier = modifier.store_light_state(false);
        }
        modifier
    }
}

pub fn set(arg: Set) {
    let responses = match util::get_bridge().set_scene(&arg.id, &arg.to_modifier()) {
        Ok(v) => v,
        Err(e) => exit!("Failed to set scene", e),
    };
    for i in responses {
        println!("{}", i);
    }
}

#[derive(Debug, StructOpt)]
pub struct Get {
    /// Identifier of the scene, if omitted all scenes are selected
    pub id: Option<String>,
}

pub fn get(arg: Get) {
    let bridge = util::get_bridge();
    match arg.id {
        Some(v) => match bridge.get_scene(&v) {
            Ok(v) => println!("{}", SceneDisplay(v)),
            Err(e) => exit!("Failed to get scene", e),
        },
        None => match bridge.get_all_scenes() {
            Ok(v) => {
                for scene in v {
                    println!("{}\n", SceneDisplay(scene));
                }
            }
            Err(e) => exit!("Failed to get scenes", e),
        },
    };
}

#[derive(Debug, StructOpt)]
pub struct Create {
    /// The name of the new scene
    name: String,
    /// Sets the identifiers of the lights that will be in this scene
    #[structopt(long, short)]
    lights: Vec<String>,
    /// Sets the type of the scene
    #[structopt(long, short, case_insensitive = true, possible_values = value::SceneType::variants())]
    kind: Option<value::SceneType>,
    /// Sets the app version of the scene
    #[structopt(long)]
    app_version: Option<i8>,
    /// Sets the app data of the scene
    #[structopt(long)]
    app_data: Option<String>,
}

impl Create {
    pub fn to_creator(&self) -> scene::Creator {
        let mut creator = scene::Creator::new(&self.name, self.lights.clone());
        if let Some(v) = &self.kind {
            creator = creator.kind(v.0);
        }
        if let Some(v) = self.app_version {
            creator = creator.app_version(v);
        }
        if let Some(v) = &self.app_data {
            creator = creator.app_data(v);
        }
        creator
    }
}

pub fn create(arg: Create) {
    match util::get_bridge().create_scene(&arg.to_creator()) {
        Ok(v) => println!("Created scene {}", v),
        Err(e) => exit!("Failed to create scene", e),
    };
}

#[derive(Debug, StructOpt)]
pub struct Delete {
    /// Identifier of the scene
    pub id: String,
}

pub fn delete(arg: Delete) {
    match util::get_bridge().delete_scene(&arg.id) {
        Ok(_) => println!("Deleted scene {}", arg.id),
        Err(e) => exit!("Failed to delete scene", e),
    };
}

struct SceneDisplay(resource::Scene);

impl fmt::Display for SceneDisplay {
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
