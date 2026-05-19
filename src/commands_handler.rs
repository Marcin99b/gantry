use std::convert::TryFrom;
use std::io::Write;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use log::error;

use crate::commands;
use crate::commands::models::{Command, CommandType};

pub fn handle(listener: TcpListener) {
    for incoming in listener.incoming() {
        let Ok(mut stream) = incoming else { continue };
        if let Some(command) = map_command(&stream) {
            let response = match command.command_type {
                CommandType::Ping => commands::ping::handle(command),
                CommandType::GetTopics => commands::get_topics::handle(command),
                CommandType::CreateTopic => commands::create_topic::handle(command),
                CommandType::DeleteTopic => commands::delete_topic::handle(command),
                CommandType::PutMessage => commands::put_message::handle(command),
                CommandType::GetMessage => commands::get_message::handle(command),
                CommandType::SubscribeTopic => commands::subscribe_topic::handle(command),
                CommandType::UnsubscribeTopic => commands::unsubscribe_topic::handle(command),
            };

            if let Some(data) = response {
                stream.write_all(&data).unwrap();
            }
        }
    }
}

fn map_command(stream: &TcpStream) -> Option<Command> {
    let full_data = read_stream_to_end(stream);
    if full_data.is_empty() {
        return None;
    }

    match CommandType::try_from(full_data[0]) {
        Ok(command_type) => Some(Command {
            command_type,
            data: full_data[1..].to_vec(),
        }),
        Err(_) => None,
    }
}

fn read_stream_to_end(mut stream: &TcpStream) -> Vec<u8> {
    let mut request_buffer = vec![];
    loop {
        let mut buf = [0u8; 1024];
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => request_buffer.extend_from_slice(&buf[..n]),
            Err(x) => {
                error!("{}", x);
                break;
            }
        }
    }
    request_buffer
}
