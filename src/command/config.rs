use crate::{arg::subcommand, util};
use std::fmt;

struct Config(huelib::Config);

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("Name: {}\n", self.0.name));
        output.push_str(&format!("SoftwareVersion: {}\n", self.0.software_version));
        output.push_str("SoftwareUpdate:\n");
        output.push_str(&format!("    State: {:?}\n", self.0.software_update.state));
        output.push_str(&format!("    Check: {}\n", self.0.software_update.check));
        output.push_str(&format!(
            "    AutoInstallOn: {}\n",
            self.0.software_update.auto_install.on
        ));
        if let Some(v) = self.0.software_update.auto_install.update_time {
            output.push_str(&format!("    AutoInstallUpdateTime: {:?}\n", v));
        }
        if let Some(v) = self.0.software_update.last_change {
            output.push_str(&format!("    LastChange: {}\n", v));
        }
        if let Some(v) = self.0.software_update.last_install {
            output.push_str(&format!("    LastInstall: {}\n", v));
        }
        output.push_str(&format!("ApiVersion: {}\n", self.0.api_version));
        output.push_str(&format!("LinkButton: {}\n", self.0.link_button));
        output.push_str(&format!("IpAddress: {}\n", self.0.ip_address));
        output.push_str(&format!("MacAddress: {}\n", self.0.mac_address));
        output.push_str(&format!("Netmask: {}\n", self.0.netmask));
        output.push_str(&format!("Gateway: {}\n", self.0.gateway));
        output.push_str(&format!("Dhcp: {}\n", self.0.dhcp));
        output.push_str(&format!("PortalServices: {}\n", self.0.portal_services));
        output.push_str(&format!(
            "PortalConnection: {:?}\n",
            self.0.portal_connection
        ));
        output.push_str("PortalState:\n");
        output.push_str(&format!("    Signedon: {}\n", self.0.portal_state.signedon));
        output.push_str(&format!("    Incoming: {}\n", self.0.portal_state.incoming));
        output.push_str(&format!("    Outgoing: {}\n", self.0.portal_state.outgoing));
        output.push_str(&format!(
            "    Communication: {:?}\n",
            self.0.portal_state.communication
        ));
        output.push_str("InternetServices:\n");
        output.push_str(&format!(
            "    Internet: {:?}\n",
            self.0.internet_services.internet
        ));
        output.push_str(&format!(
            "    RemoteAccess: {:?}\n",
            self.0.internet_services.remote_access
        ));
        output.push_str(&format!("    Time: {:?}\n", self.0.internet_services.time));
        output.push_str(&format!(
            "    SoftwareUpdate: {:?}\n",
            self.0.internet_services.software_update
        ));
        output.push_str(&format!("CurrentTime: {}\n", self.0.current_time));
        if let Some(v) = self.0.local_time {
            output.push_str(&format!("LocalTime: {}\n", v));
        }
        if let Some(v) = &self.0.timezone {
            output.push_str(&format!("Timezone: {}\n", v));
        }
        output.push_str(&format!("ZigbeeChannel: {}\n", self.0.zigbee_channel));
        output.push_str(&format!("ModelId: {}\n", self.0.model_id));
        output.push_str(&format!("BridgeId: {}\n", self.0.bridge_id));
        output.push_str(&format!("FactoryNew: {}\n", self.0.factory_new));
        if let Some(v) = &self.0.replaces_bridge_id {
            output.push_str(&format!("ReplacesBridgeId: {}\n", v));
        }
        output.push_str(&format!("DatastoreVersion: {}\n", self.0.datastore_version));
        output.push_str(&format!("StarterkitId: {}\n", self.0.starterkit_id));
        output.push_str(&format!("BackupStatus: {:?}\n", self.0.backup.status));
        output.push_str(&format!("BackupError: {:?}\n", self.0.backup.error));
        output.push_str("Whitelist:\n");
        for user in self.0.whitelist.iter() {
            output.push_str(&format!("    User {}:\n", user.id));
            output.push_str(&format!("        Name: {}\n", user.name));
            output.push_str(&format!("        LastUseDate: {}\n", user.last_use_date));
            output.push_str(&format!("        CreateDate: {}\n", user.create_date));
        }
        output.pop();
        write!(f, "{}", output)
    }
}

pub fn set(arg: subcommand::config::Set) {
    let responses = match util::get_bridge().set_config(&arg.to_modifier()) {
        Ok(v) => v,
        Err(e) => exit!("Failed to set config", e),
    };
    for i in responses {
        println!("{}", i);
    }
}

pub fn get(_arg: subcommand::config::Get) {
    let bridge = util::get_bridge();
    match bridge.get_config() {
        Ok(v) => println!("{}", Config(v)),
        Err(e) => exit!("Failed to get scene", e),
    };
}
