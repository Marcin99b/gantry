use std::{fs::OpenOptions, io::Write};

use log::info;

use crate::commands::models::Command;

static SEPARATOR: &[u8] = b"\r\n\r\n\r\n";

// todo get topic id
// first int = offset
// rest = data
pub fn handle(command: Command) -> Option<Vec<u8>> {
    info!("Starting PUT");
    let mut buffer = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data.txt")
        .unwrap();

    buffer.write_all(&command.data).unwrap();
    buffer.write_all(SEPARATOR).unwrap();

    None
}
