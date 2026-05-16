use std::{fs::OpenOptions, io::Write};

use crate::commands::models::Command;

pub fn handle(command: Command) {
    let mut buffer = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data.txt")
        .unwrap();

    buffer.write_all(&command.data).unwrap();

    buffer.write_all("\r\n\r\n\r\n".as_bytes()).unwrap();
}
