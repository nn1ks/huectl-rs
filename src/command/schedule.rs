use crate::{arg::subcommand, util};
use std::fmt;

struct Schedule(huelib::Schedule);

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("Schedule {}:\n", self.0.id));
        output.push_str(&format!("    Name: {}\n", self.0.name));
        output.push_str(&format!("    Description: {}\n", self.0.description));
        output.push_str(&format!("    CommandAddress: {}\n", self.0.command.address));
        output.push_str(&format!(
            "    CommandRequestType: {:?}\n",
            self.0.command.request_type
        ));
        output.push_str(&format!("    CommandBody: {:?}\n", self.0.command.body));
        output.push_str(&format!("    LocalTime: {}\n", self.0.local_time));
        if let Some(v) = self.0.start_time {
            output.push_str(&format!("    StartTime: {}\n", v));
        }
        output.push_str(&format!("    Status: {:?}\n", self.0.status));
        if let Some(v) = self.0.auto_delete {
            output.push_str(&format!("    AutoDelete: {}\n", v));
        }
        output.pop();
        write!(f, "{}", output)
    }
}

pub fn set(arg: subcommand::schedule::Set) {
    let responses = match util::get_bridge().set_schedule(&arg.id, &arg.to_modifier()) {
        Ok(v) => v,
        Err(e) => exit!("Failed to set schedule", e),
    };
    for i in responses {
        println!("{}", i);
    }
}

pub fn get(arg: subcommand::schedule::Get) {
    let bridge = util::get_bridge();
    match arg.id {
        Some(v) => match bridge.get_schedule(&v) {
            Ok(v) => println!("{}", Schedule(v)),
            Err(e) => exit!("Failed to get schedule", e),
        },
        None => match bridge.get_all_schedules() {
            Ok(v) => {
                for schedule in v {
                    println!("{}\n", Schedule(schedule));
                }
            }
            Err(e) => exit!("Failed to get schedules", e),
        },
    };
}

pub fn create(arg: subcommand::schedule::Create) {
    match util::get_bridge().create_schedule(&arg.to_creator()) {
        Ok(v) => println!("Created schedule {}", v),
        Err(e) => exit!("Failed to create schedule", e),
    };
}

pub fn delete(arg: subcommand::schedule::Delete) {
    match util::get_bridge().delete_schedule(&arg.id) {
        Ok(_) => println!("Deleted schedule {}", arg.id),
        Err(e) => exit!("Failed to delete schedule", e),
    };
}
