use crate::{arg::subcommand, util};

pub fn get(arg: subcommand::capabilities::Get) {
    match util::get_bridge().get_capabilities() {
        Ok(v) => {
            if arg.json {
                match serde_json::to_string_pretty(&v) {
                    Ok(v) => println!("{}", v),
                    Err(e) => util::print_err("Failed to serialize data", e),
                };
            } else {
                // TODO: Create struct for pretty printing
                println!("{:#?}", v);
            }
        }
        Err(e) => util::print_err("Failed to get capabilities", e),
    };
}
