mod commands;
mod commands_handler;

use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    net::TcpListener,
};

use log::{Level, LevelFilter, Metadata, Record, error, info, set_max_level, warn};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

fn main() {
    if let Ok(()) = log::set_logger(&LOGGER) {
        set_max_level(LevelFilter::Info)
    }

    let file_path = "data.txt";
    let listener = TcpListener::bind("127.0.0.1:2137").expect("Failed to bind TCP listener");
    commands_handler::handle(listener);
    // loop {
    //     match listener.incoming().next() {
    //         Some(Ok(mut stream)) => {
    //             println!("Connection from: {}", stream.peer_addr().unwrap().ip());
    //             let mut buf = [0u8; 4096];
    //             match stream.read(&mut buf) {
    //                 Ok(0) => {
    //                     warn!("Empty stream");
    //                 }
    //                 Ok(n) => match str::from_utf8(&buf[..n]) {
    //                     Ok(request) => {
    //                         info!("Request: {}", request);
    //                         let response = handle_request(request, file_path);
    //                         info!("Response: {}", response);
    //                         let bytes = response.as_bytes();
    //                         stream.write_all(bytes).unwrap();
    //                     }
    //                     Err(x) => error!("{}", x),
    //                 },
    //                 Err(x) => error!("{}", x),
    //             }
    //         }
    //         Some(Err(x)) => error!("{}", x),
    //         None => continue,
    //     }
    // }
}

fn handle_request(request: &str, file_path: &str) -> String {
    let mut splitted_request = request.split_whitespace();
    let method = splitted_request.next();
    let body = splitted_request.next();

    match method {
        Some("put") => {
            if let Some(x) = body {
                handle_put(x, file_path)
            } else {
                "ERROR: 001; Cannot handle put command".to_string()
            }
        }
        Some("get") => {
            if let Some(x) = body {
                handle_get(x, file_path)
            } else {
                "ERROR: 002; Cannot handle get command".to_string()
            }
        }
        Some("maxoffset") => handle_maxoffset(file_path),
        Some(parsed) => format!("Method {} not found", parsed).to_string(),
        None => "Cannot handle command".to_string(),
    }
}

fn handle_put(command: &str, file_path: &str) -> String {
    let mut buffer = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();

    buffer
        .write_all(format!("{}\n", command).as_bytes())
        .unwrap();
    "Ok".to_string()
}

fn handle_get(command: &str, file_path: &str) -> String {
    let offset = command.parse::<usize>().unwrap();

    let mut buffer = File::open(file_path).unwrap();
    let mut buf = String::new();
    buffer.read_to_string(&mut buf).unwrap();

    let line = buf.lines().nth(offset).unwrap();
    line.to_string()
}

fn handle_maxoffset(file_path: &str) -> String {
    let mut buffer = File::open(file_path).unwrap();
    let mut buf = String::new();
    buffer.read_to_string(&mut buf).unwrap();

    let lines_count = buf.lines().count();
    if lines_count == 0 {
        return "ERROR: 003; No messages found".to_string();
    }

    let max_offset = lines_count - 1;
    max_offset.to_string()
}
