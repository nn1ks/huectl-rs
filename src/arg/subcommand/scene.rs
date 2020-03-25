use crate::arg::value;
use huelib::Modifier;
use structopt::StructOpt;

/// Modifies the state and attributes of a scene
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
    pub fn to_modifier(&self) -> huelib::scene::Modifier {
        let mut modifier = huelib::scene::Modifier::new();
        if let Some(v) = &self.name {
            modifier = modifier.name(v);
        }
        if let Some(v) = &self.lights {
            let lights: Vec<&str> = v.iter().map(AsRef::as_ref).collect();
            modifier = modifier.lights(&lights);
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
pub struct Get {
    /// Identifier of the scene, if omitted all scenes are selected
    pub id: Option<String>,
}

/// Creates a scene
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
    pub fn to_creator(&self) -> huelib::scene::Creator {
        let lights: Vec<&str> = self.lights.iter().map(|v| v.as_ref()).collect();
        let mut creator = huelib::scene::Creator::new(&self.name, &lights);
        if let Some(v) = &self.kind {
            creator = creator.kind(v.value);
        }
        let mut app_data = huelib::scene::AppData::new();
        if let Some(v) = self.app_version {
            app_data = app_data.version(v);
        }
        if let Some(v) = &self.app_data {
            app_data = app_data.data(v.clone());
        }
        if app_data.version != None || app_data.data != None {
            creator = creator.app_data(app_data);
        }
        creator
    }
}

/// Deletes a scene
#[derive(Debug, StructOpt)]
pub struct Delete {
    /// Identifier of the scene
    pub id: String,
}
