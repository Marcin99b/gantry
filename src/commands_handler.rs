use std::convert::TryFrom;
use std::io::{self, Write};
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use crate::commands;
use crate::commands::models::{Command, CommandType};

static OK: &[u8] = b"OK";

pub fn handle(listener: TcpListener) {
    for incoming in listener.incoming() {
        let Ok(mut stream) = incoming else { continue };
        if let Ok(command) = read_command(&mut stream) {
            let response = execute(command);
            let data = response.unwrap_or_else(|| OK.to_vec());
            write_response(&mut stream, &data).unwrap();
        }
    }
}

/*
 * planned format
 *
 * 1B = command type
 * rest = data for command handler
 *
 * GET = 1B topic | rest data
 * PUT = 1B topic | rest data
 *
 * CREATE TOPIC = 1B topic
 * DELETE TOPIC = 1B topic
 * GET TOPICS = empty
 *
 * SUBSCRIBE = 1B topic
 * UNSUBSCRIBE = 1B topic
 *
 * PING = empty
 */

fn read_command(stream: &mut TcpStream) -> io::Result<Command> {
    let mut header = [0u8; 5];
    stream.read_exact(&mut header)?;

    let command_type = CommandType::try_from(header[0]).unwrap();

    let data_len = u32::from_le_bytes(header[1..5].try_into().unwrap()) as usize;
    let mut data = vec![0u8; data_len];
    if data_len > 0 {
        stream.read_exact(&mut data)?;
    }

    Ok(Command { command_type, data })
}

fn write_response(stream: &mut TcpStream, data: &[u8]) -> io::Result<()> {
    let len = (data.len() as u32).to_le_bytes();
    stream.write_all(&len)?;
    stream.write_all(data)?;
    stream.flush()
}

fn execute(command: Command) -> Option<Vec<u8>> {
    match command.command_type {
        CommandType::Ping => commands::ping::handle(command),
        CommandType::GetTopics => commands::get_topics::handle(command),
        CommandType::CreateTopic => commands::create_topic::handle(command),
        CommandType::DeleteTopic => commands::delete_topic::handle(command),
        CommandType::PutMessage => commands::put_message::handle(command),
        CommandType::GetMessage => commands::get_message::handle(command),
        CommandType::SubscribeTopic => commands::subscribe_topic::handle(command),
        CommandType::UnsubscribeTopic => commands::unsubscribe_topic::handle(command),
    }
}
