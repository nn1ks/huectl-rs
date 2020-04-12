use crate::arg::value;
use structopt::StructOpt;

/// Modifies attributes of a schedule
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
    pub fn to_modifier(&self) -> huelib::schedule::Modifier {
        let mut modifier = huelib::schedule::Modifier::new();
        if self.on {
            modifier = modifier.status(huelib::schedule::Status::Enabled);
        } else if self.off {
            modifier = modifier.status(huelib::schedule::Status::Disabled);
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

/// Prints attributes of a schedule
#[derive(Debug, StructOpt)]
pub struct Get {
    /// Identifier of the schedule, if omitted all schedules are selected
    pub id: Option<String>,
}

/// Creates a schedule
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
    pub fn to_creator(&self) -> huelib::schedule::Creator {
        let mut creator = huelib::schedule::Creator::new(
            huelib::schedule::Command {
                address: self.address.clone(),
                request_type: self.request_type.value,
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
            creator = creator.status(huelib::schedule::Status::Enabled);
        } else if self.off {
            creator = creator.status(huelib::schedule::Status::Disabled);
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

/// Deletes a schedule
#[derive(Debug, StructOpt)]
pub struct Delete {
    /// Identifier of the schedule
    pub id: String,
}
