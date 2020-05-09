use crate::{output::Rule as OutputRule, util};
use huelib::resource::{rule, Modifier};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Arg {
    /// Modifies attributes of a rule
    Set(Set),
    /// Prints attributes of a rule
    Get(Get),
    /// Creates a rule
    Create(Create),
    /// Deletes a rule
    Delete(Delete),
}

#[derive(Debug, StructOpt)]
pub struct Set {
    /// Identifier of the rule
    pub id: String,
    /// Sets the name of the rule
    #[structopt(long, short)]
    name: Option<String>,
}

impl Set {
    pub fn to_modifier(&self) -> rule::Modifier {
        let mut modifier = rule::Modifier::new();
        if let Some(v) = &self.name {
            modifier = modifier.name(v);
        }
        modifier
    }
}

pub fn set(arg: Set) {
    let responses = match util::get_bridge().set_rule(&arg.id, &arg.to_modifier()) {
        Ok(v) => v,
        Err(e) => exit!("Failed to set rule", e),
    };
    for i in responses {
        println!("{}", i);
    }
}

#[derive(Debug, StructOpt)]
pub struct Get {
    /// Identifier of the rule, if omitted all rules are selected
    pub id: Option<String>,
}

pub fn get(arg: Get) {
    let bridge = util::get_bridge();
    match arg.id {
        Some(v) => match bridge.get_rule(&v) {
            Ok(v) => println!(
                "{}",
                serde_json::to_string_pretty(&OutputRule::from(v)).unwrap()
            ),
            Err(e) => exit!("Failed to get rule", e),
        },
        None => match bridge.get_all_rules() {
            Ok(v) => {
                let rules: Vec<OutputRule> = v.into_iter().map(OutputRule::from).collect();
                println!("{}", serde_json::to_string_pretty(&rules).unwrap());
            }
            Err(e) => exit!("Failed to get rules", e),
        },
    };
}

#[derive(Debug, StructOpt)]
pub struct Create {
    /// The name of the new rule
    name: String,
}

impl Create {
    pub fn to_creator(&self) -> rule::Creator {
        rule::Creator::new(Vec::new(), Vec::new()).name(&self.name)
    }
}

pub fn create(arg: Create) {
    match util::get_bridge().create_rule(&arg.to_creator()) {
        Ok(v) => println!("Created rule {}", v),
        Err(e) => exit!("Failed to create rule", e),
    }
}

#[derive(Debug, StructOpt)]
pub struct Delete {
    /// Identifier of the rule
    pub id: String,
}

pub fn delete(arg: Delete) {
    match util::get_bridge().delete_rule(&arg.id) {
        Ok(_) => println!("Deleted rule {}", arg.id),
        Err(e) => exit!("Failed to delete rule", e),
    };
}
