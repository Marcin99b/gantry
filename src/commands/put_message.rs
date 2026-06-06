use std::{fs::OpenOptions, io::Write};

use log::info;

use crate::commands::models::Command;

static SEPARATOR: &[u8] = b"\r\n\r\n\r\n";

// first uint = topic id
// rest = data
pub fn handle(command: Command) -> Option<Vec<u8>> {
    //todo check if topic exist
    info!("Starting PUT");
    let topic_id = u32::from_le_bytes(command.data[..4].try_into().unwrap());
    let mut buffer = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("{}.txt", topic_id))
        .unwrap();

    buffer.write_all(&command.data[4..]).unwrap();
    buffer.write_all(SEPARATOR).unwrap();

    None
}
