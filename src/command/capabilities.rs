use crate::{arg::subcommand, util};

pub fn get(_arg: subcommand::capabilities::Get) {
    match util::get_bridge().get_capabilities() {
        Ok(v) => {
            // TODO: Create struct for printing
            println!("{:#?}", v);
        }
        Err(e) => exit!("Failed to get capabilities", e),
    };
}
