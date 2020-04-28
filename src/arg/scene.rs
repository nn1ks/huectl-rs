use crate::{arg::value, output::Scene as OutputScene, util};
use huelib::resource::{scene, Modifier};
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
            Ok(v) => println!(
                "{}",
                serde_json::to_string_pretty(&OutputScene::from(v)).unwrap()
            ),
            Err(e) => exit!("Failed to get scene", e),
        },
        None => match bridge.get_all_scenes() {
            Ok(v) => {
                let scenes: Vec<OutputScene> = v.into_iter().map(OutputScene::from).collect();
                println!("{}", serde_json::to_string_pretty(&scenes).unwrap());
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
