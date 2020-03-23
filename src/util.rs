use crate::config;

pub fn print_err<E: std::error::Error>(description: &str, error: E) -> ! {
    eprintln!("{}: {}", description, error);
    std::process::exit(1)
}

pub fn get_bridge() -> huelib::bridge::Bridge {
    match config::get() {
        Ok(v) => huelib::bridge::Bridge::new(v.bridge_ip, &v.bridge_username),
        Err(e) => print_err("Failed to get configuration environment variables", e),
    }
}
