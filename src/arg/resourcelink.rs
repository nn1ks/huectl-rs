use crate::{output::Resourcelink as OutputResourcelink, util};
use huelib::resource::{resourcelink, Modifier};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Arg {
    /// Modifies attributes of a resourcelink
    Set(Set),
    /// Prints the attributes of a resourcelink
    Get(Get),
    /// Creates a resourcelink
    Create(Create),
    /// Deletes a resourcelink
    Delete(Delete),
}

#[derive(Debug, StructOpt)]
pub struct Set {
    /// Identifier of the resourcelink
    pub id: String,
    /// Sets the name of the resourcelink
    #[structopt(long, short)]
    name: Option<String>,
    /// Sets the description of the resourcelink
    #[structopt(long, short)]
    description: Option<String>,
    /// Sets the class id of the resourcelink
    #[structopt(long, short)]
    class_id: Option<u16>,
}

impl Set {
    pub fn to_modifier(&self) -> resourcelink::Modifier {
        let mut modifier = resourcelink::Modifier::new();
        if let Some(v) = &self.name {
            modifier = modifier.name(v);
        }
        if let Some(v) = &self.description {
            modifier = modifier.description(v);
        }
        if let Some(v) = self.class_id {
            modifier = modifier.class_id(v);
        }
        modifier
    }
}

pub fn set(arg: Set) {
    let responses = match util::get_bridge().set_resourcelink(&arg.id, &arg.to_modifier()) {
        Ok(v) => v,
        Err(e) => exit!("Failed to set resourcelink", e),
    };
    for i in responses {
        println!("{}", i);
    }
}

#[derive(Debug, StructOpt)]
pub struct Get {
    /// Identifier of the resourcelink, if ommited all resourcelinks are selected
    id: Option<String>,
}

pub fn get(arg: Get) {
    let bridge = util::get_bridge();
    match arg.id {
        Some(v) => match bridge.get_resourcelink(&v) {
            Ok(v) => println!(
                "{}",
                serde_json::to_string_pretty(&OutputResourcelink::from(v)).unwrap()
            ),
            Err(e) => exit!("Failed to get resourcelinks", e),
        },
        None => match bridge.get_all_resourcelinks() {
            Ok(v) => {
                let resourcelinks: Vec<OutputResourcelink> =
                    v.into_iter().map(OutputResourcelink::from).collect();
                println!("{}", serde_json::to_string_pretty(&resourcelinks).unwrap());
            }
            Err(e) => exit!("Failed to get resourcelinks", e),
        },
    };
}

#[derive(Debug, StructOpt)]
pub struct Create {
    /// The name of the new resourcelink
    name: String,
    /// The class id of the new resourcelink
    class_id: u16,
    /// Sets the description of the resourcelink
    #[structopt(long, short)]
    description: Option<String>,
    /// Sets the owner of the resourcelink
    #[structopt(long, short)]
    owner: Option<String>,
    /// Enables automatic removal of the resourcelink if not referenced anymore
    #[structopt(long, short)]
    recycle: bool,
    /// Disables automatic removal of the resourcelink if not referenced anymore
    #[structopt(long, short = "R")]
    no_recycle: bool,
}

impl Create {
    pub fn to_creator(&self) -> resourcelink::Creator {
        let mut creator = resourcelink::Creator::new(&self.name, self.class_id);
        if let Some(v) = &self.description {
            creator = creator.description(v);
        }
        if let Some(v) = &self.owner {
            creator = creator.owner(v);
        }
        if self.recycle {
            creator = creator.recycle(true);
        } else if self.no_recycle {
            creator = creator.recycle(false);
        }
        creator
    }
}

pub fn create(arg: Create) {
    match util::get_bridge().create_resourcelink(&arg.to_creator()) {
        Ok(v) => println!("Created resourcelink {}", v),
        Err(e) => exit!("Failed to create resourcelink", e),
    }
}

#[derive(Debug, StructOpt)]
pub struct Delete {
    /// Identifier of the resourcelink
    pub id: String,
}

pub fn delete(arg: Delete) {
    match util::get_bridge().delete_resourcelink(&arg.id) {
        Ok(_) => println!("Deleted resourcelink {}", arg.id),
        Err(e) => exit!("Failed to delete resourcelink", e),
    };
}
