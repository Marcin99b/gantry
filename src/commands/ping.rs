use crate::commands::models::Command;

pub fn handle(_: Command) -> Option<Vec<u8>> {
    Some(vec![1])
}
