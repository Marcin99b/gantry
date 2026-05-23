use log::{error, info, warn};
use std::convert::TryFrom;
use std::error::Error;
use std::io::{self, Write};
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

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
                info!("Response length: {}", data.len());
                stream.write_all(&data).unwrap();
            } else {
                let mut empty_array = [0u8; 1];
                empty_array[0] = 1;
                stream.write_all(&empty_array).unwrap();
            }
        }
    }
}

fn map_command(stream: &TcpStream) -> Option<Command> {
    match read_stream_to_end(stream) {
        Ok(full_data) => {
            if full_data.is_empty() {
                warn!("Stream was empty");
                return None;
            }
            match CommandType::try_from(full_data[0]) {
                Ok(command_type) => Some(Command {
                    command_type,
                    data: full_data[1..].to_vec(),
                }),
                Err(x) => {
                    error!("{}", x);
                    None
                }
            }
        }
        Err(x) => {
            error!("{}", x);
            None
        }
    }
}

//todo return Result<>
fn read_stream_to_end(mut stream: &TcpStream) -> io::Result<Vec<u8>> {
    let request_limit = 1024 * 10;
    let mut request_buffer = vec![];
    loop {
        let mut buf = [0u8; 1024];
        match stream.read(&mut buf) {
            Ok(0) => {
                info!("Incoming bytes: 0");
                break;
            }
            Ok(n) => {
                info!("Incoming bytes: {}", n);
                if request_buffer.len() + n > request_limit {
                    panic!("Request length exceeded limit {}", request_limit);
                }
                request_buffer.extend_from_slice(&buf[..n])
            }
            Err(x) => {
                error!("{}", x);
                break;
            }
        }
    }

    Ok(request_buffer)
}
