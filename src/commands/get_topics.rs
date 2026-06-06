use std::{fs::File, io::Read};

use crate::commands::models::Command;

pub fn handle(_: Command) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    File::open("topics.txt")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();

    // todo split by separator
    // return ID + topic name
    None
}
