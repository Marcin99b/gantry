use std::{fs::OpenOptions, io::Write};

use crate::commands::models::Command;

static SEPARATOR: &[u8] = b"\r\n\r\n\r\n";

pub fn handle(command: Command) -> Option<Vec<u8>> {
    let mut buffer = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data.txt")
        .unwrap();

    buffer.write_all(&command.data).unwrap();
    buffer.write_all(SEPARATOR).unwrap();
    None
}
