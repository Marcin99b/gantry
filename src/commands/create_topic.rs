use std::{fs::OpenOptions, io::Write};

use crate::commands::models::Command;
static SEPARATOR: &[u8] = b"\r\n";

pub fn handle(command: Command) -> Option<Vec<u8>> {
    // todo check if topic exist
    // todo generate topic id
    // todo return topic id
    let mut buffer = OpenOptions::new()
        .append(true)
        .create(true)
        .open("topics.txt")
        .unwrap();
    buffer.write_all(&command.data).unwrap();
    buffer.write_all(SEPARATOR).unwrap();

    None
}
