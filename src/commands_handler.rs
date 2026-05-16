use std::convert::TryFrom;
use std::{
    io::{Error, Read},
    net::{TcpListener, TcpStream},
};

use log::error;

pub enum CommandType {
    Ping,
    GetTopics,
    CreateTopic,
    DeleteTopic,
    PutMessage,
    GetMessage,
    SubscribeTopic,
    UnsubscribeTopic,
}

impl TryFrom<u8> for CommandType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CommandType::Ping),
            1 => Ok(CommandType::GetTopics),
            2 => Ok(CommandType::CreateTopic),
            3 => Ok(CommandType::DeleteTopic),
            4 => Ok(CommandType::PutMessage),
            5 => Ok(CommandType::GetMessage),
            6 => Ok(CommandType::SubscribeTopic),
            7 => Ok(CommandType::UnsubscribeTopic),
            v => Err(v),
        }
    }
}

pub struct Command {
    command_type: CommandType,
    data: Vec<u8>,
}

pub fn handle(listener: TcpListener) {
    for incoming in listener.incoming() {
        if let Some(command) = map_command(incoming) {}
    }
}

fn map_command(incoming: Result<TcpStream, Error>) -> Option<Command> {
    match incoming {
        Ok(mut stream) => {
            let mut buf = [0u8; 4096];
            match stream.read(&mut buf) {
                Ok(0) => None,
                Ok(n) => match CommandType::try_from(buf[0]) {
                    Ok(command_type) => Some(Command {
                        command_type,
                        data: buf[1..n].to_vec(),
                    }),
                    Err(_) => None,
                },
                Err(x) => {
                    error!("{}", x);
                    None
                }
            }
        }
        _ => None,
    }
}

