use crate::{arg::subcommand, util};
use std::fmt;

struct Capabilities(huelib::Capabilities);

impl fmt::Display for Capabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str("Lights:\n");
        output.push_str(&format!("    Available: {}\n", self.0.lights.available));
        output.push_str(&format!("    Total: {}\n", self.0.lights.total));
        output.push_str("Groups:\n");
        output.push_str(&format!("    Available: {}\n", self.0.groups.available));
        output.push_str(&format!("    Total: {}\n", self.0.groups.total));
        output.push_str("Sensors:\n");
        output.push_str(&format!("    Available: {}\n", self.0.sensors.available));
        output.push_str(&format!("    Total: {}\n", self.0.sensors.total));
        output.push_str("    Clip:\n");
        output.push_str(&format!(
            "        Available: {}\n",
            self.0.sensors.clip.available
        ));
        output.push_str(&format!("        Total: {}\n", self.0.sensors.clip.total));
        output.push_str("    Zll:\n");
        output.push_str(&format!(
            "        Available: {}\n",
            self.0.sensors.zll.available
        ));
        output.push_str(&format!("        Total: {}\n", self.0.sensors.zll.total));
        output.push_str("    Zgp:\n");
        output.push_str(&format!(
            "        Available: {}\n",
            self.0.sensors.zgp.available
        ));
        output.push_str(&format!("        Total: {}\n", self.0.sensors.zgp.total));
        output.push_str("Scenes:\n");
        output.push_str(&format!("    Available: {}\n", self.0.scenes.available));
        output.push_str(&format!("    Total: {}\n", self.0.scenes.total));
        output.push_str("    LightStates:\n");
        output.push_str(&format!(
            "        Available: {}\n",
            self.0.scenes.light_states.available
        ));
        output.push_str(&format!(
            "        Total: {}\n",
            self.0.scenes.light_states.total
        ));
        output.push_str("Schedules:\n");
        output.push_str(&format!("    Available: {}\n", self.0.schedules.available));
        output.push_str(&format!("    Total: {}\n", self.0.schedules.total));
        output.push_str("Rules:\n");
        output.push_str(&format!("    Available: {}\n", self.0.rules.available));
        output.push_str(&format!("    Total: {}\n", self.0.rules.total));
        output.push_str("    Conditions:\n");
        output.push_str(&format!(
            "        Available: {}\n",
            self.0.rules.conditions.available
        ));
        output.push_str(&format!(
            "        Total: {}\n",
            self.0.rules.conditions.total
        ));
        output.push_str("    Actions:\n");
        output.push_str(&format!(
            "        Available: {}\n",
            self.0.rules.actions.available
        ));
        output.push_str(&format!("        Total: {}\n", self.0.rules.actions.total));
        output.push_str("Resourcelinks:\n");
        output.push_str(&format!(
            "    Available: {}\n",
            self.0.resourcelinks.available
        ));
        output.push_str(&format!("    Total: {}\n", self.0.resourcelinks.total));
        output.push_str("Streaming:\n");
        output.push_str(&format!("    Available: {}\n", self.0.streaming.available));
        output.push_str(&format!("    Total: {}\n", self.0.streaming.total));
        output.push_str(&format!("    Channels: {}\n", self.0.streaming.total));
        output.pop();
        write!(f, "{}", output)
    }
}

pub fn get(_arg: subcommand::capabilities::Get) {
    match util::get_bridge().get_capabilities() {
        Ok(v) => println!("{}", Capabilities(v)),
        Err(e) => exit!("Failed to get capabilities", e),
    };
}
