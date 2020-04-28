use crate::{arg::value, output::Schedule as OutputSchedule, util};
use huelib::resource::{self, schedule, Modifier};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Arg {
    /// Modifies attributes of a schedule
    Set(Set),
    /// Prints attributes of a schedule
    Get(Get),
    /// Creates a schedule
    Create(Create),
    /// Deletes a schedule
    Delete(Delete),
}

#[derive(Debug, StructOpt)]
pub struct Set {
    /// Identifier of the schedule
    pub id: String,
    /// Enables the schedule
    #[structopt(long)]
    on: bool,
    /// Disables the schedule
    #[structopt(long)]
    off: bool,
    /// Renames the schedule
    #[structopt(long, short)]
    name: Option<String>,
    /// Sets the description of the schedule
    #[structopt(long, short)]
    description: Option<String>,
    /// Sets the time when the command will be executed
    #[structopt(long, short)]
    time: Option<String>,
    /// Enables automatic removal of the schedule if expired
    #[structopt(long, short)]
    auto_delete: bool,
    /// Disables automatic removal of the schedule if expired
    #[structopt(long, short = "A")]
    no_auto_delete: bool,
}

impl Set {
    pub fn to_modifier(&self) -> schedule::Modifier {
        let mut modifier = schedule::Modifier::new();
        if self.on {
            modifier = modifier.status(schedule::Status::Enabled);
        } else if self.off {
            modifier = modifier.status(schedule::Status::Disabled);
        }
        if let Some(v) = &self.name {
            modifier = modifier.name(v);
        }
        if let Some(v) = &self.description {
            modifier = modifier.description(v);
        }
        if let Some(v) = &self.time {
            modifier = modifier.localtime(v);
        }
        if self.auto_delete {
            modifier = modifier.auto_delete(true);
        } else if self.no_auto_delete {
            modifier = modifier.auto_delete(false);
        }
        modifier
    }
}

pub fn set(arg: Set) {
    let responses = match util::get_bridge().set_schedule(&arg.id, &arg.to_modifier()) {
        Ok(v) => v,
        Err(e) => exit!("Failed to set schedule", e),
    };
    for i in responses {
        println!("{}", i);
    }
}

#[derive(Debug, StructOpt)]
pub struct Get {
    /// Identifier of the schedule, if omitted all schedules are selected
    pub id: Option<String>,
}

pub fn get(arg: Get) {
    let bridge = util::get_bridge();
    match arg.id {
        Some(v) => match bridge.get_schedule(&v) {
            Ok(v) => println!(
                "{}",
                serde_json::to_string_pretty(&OutputSchedule::from(v)).unwrap()
            ),
            Err(e) => exit!("Failed to get schedule", e),
        },
        None => match bridge.get_all_schedules() {
            Ok(v) => {
                let schedules: Vec<OutputSchedule> =
                    v.into_iter().map(OutputSchedule::from).collect();
                println!("{}", serde_json::to_string_pretty(&schedules).unwrap());
            }
            Err(e) => exit!("Failed to get schedules", e),
        },
    };
}

#[derive(Debug, StructOpt)]
pub struct Create {
    /// The name of the schedule
    name: Option<String>,
    /// Sets the address of the command
    #[structopt(long)]
    address: String,
    /// Sets the request type of the command
    #[structopt(long, case_insensitive = true, possible_values = value::ScheduleRequestType::variants())]
    request_type: value::ScheduleRequestType,
    // TODO: Add option for setting the command body
    /// Sets the time when the command will be executed
    #[structopt(long, short)]
    time: String,
    /// Sets the description of the schedule
    #[structopt(long, short)]
    description: Option<String>,
    /// Enables the schedule
    #[structopt(long)]
    on: bool,
    /// Disables the schedule
    #[structopt(long)]
    off: bool,
    /// Enables automatic removal of the schedule if expired
    #[structopt(long, short)]
    auto_delete: bool,
    /// Disables automatic removal of the schedule if expired
    #[structopt(long, short = "A")]
    no_auto_delete: bool,
    /// Enables automatic removal of the schedule if not referenced anymore
    #[structopt(long, short)]
    recycle: bool,
    /// Disables automatic removal of the schedule if not referenced anymore
    #[structopt(long, short = "R")]
    no_recycle: bool,
}

impl Create {
    pub fn to_creator(&self) -> schedule::Creator {
        let mut creator = schedule::Creator::new(
            resource::Action {
                address: self.address.clone(),
                request_type: self.request_type.0,
                body: std::collections::HashMap::new(),
            },
            self.time.clone(),
        );
        if let Some(v) = &self.name {
            creator = creator.name(v);
        }
        if let Some(v) = &self.description {
            creator = creator.description(v);
        }
        if self.on {
            creator = creator.status(schedule::Status::Enabled);
        } else if self.off {
            creator = creator.status(schedule::Status::Disabled);
        }
        if self.auto_delete {
            creator = creator.auto_delete(true);
        } else if self.no_auto_delete {
            creator = creator.auto_delete(false);
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
    match util::get_bridge().create_schedule(&arg.to_creator()) {
        Ok(v) => println!("Created schedule {}", v),
        Err(e) => exit!("Failed to create schedule", e),
    };
}

#[derive(Debug, StructOpt)]
pub struct Delete {
    /// Identifier of the schedule
    pub id: String,
}

pub fn delete(arg: Delete) {
    match util::get_bridge().delete_schedule(&arg.id) {
        Ok(_) => println!("Deleted schedule {}", arg.id),
        Err(e) => exit!("Failed to delete schedule", e),
    };
}
