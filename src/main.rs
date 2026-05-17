mod commands;
mod commands_handler;
mod simple_logger;

use std::net::TcpListener;

use log::{LevelFilter, set_max_level};

static LOGGER: simple_logger::SimpleLogger = simple_logger::SimpleLogger;

fn main() {
    if let Ok(()) = log::set_logger(&LOGGER) {
        set_max_level(LevelFilter::Info)
    }

    let listener = TcpListener::bind("127.0.0.1:2137").expect("Failed to bind TCP listener");
    commands_handler::handle(listener);
}
