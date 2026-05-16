use std::convert::TryFrom;
use std::{
    io::{Error, Read},
    net::{TcpListener, TcpStream},
};

use log::error;

use crate::commands;
use crate::commands::models::{Command, CommandType};

pub fn handle(listener: TcpListener) {
    for incoming in listener.incoming() {
        if let Some(command) = map_command(incoming) {
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
