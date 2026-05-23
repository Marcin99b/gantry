use std::{fs::File, io::Read};

use log::info;

use crate::commands::models::Command;

const SEPARATOR: &[u8] = b"\r\n\r\n\r\n";

pub fn handle(command: Command) -> Option<Vec<u8>> {
    info!("Starting GET");
    let offset = u32::from_be_bytes(command.data.try_into().unwrap()) as usize;
    let mut buf = Vec::new();
    File::open("data.txt")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();

    entries(&buf).nth(offset).map(|result| result.to_owned())
}

fn entries(data: &[u8]) -> impl Iterator<Item = &[u8]> {
    let mut rest = data;
    std::iter::from_fn(move || {
        if rest.is_empty() {
            return None;
        }
        let i = rest
            .windows(SEPARATOR.len())
            .position(|w| w == SEPARATOR)
            .unwrap_or(rest.len() - SEPARATOR.len() + 1);
        let item = &rest[..i.min(rest.len())];
        rest = rest.get(i + SEPARATOR.len()..).unwrap_or(&[]);
        Some(item)
    })
}
