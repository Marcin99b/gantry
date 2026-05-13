use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    net::TcpListener,
};

//struct Message {}

fn main() {
    let file_path = "data.txt";
    let listener = TcpListener::bind("127.0.0.1:2137").expect("Failed to bind TCP listener");

    loop {
        match listener.incoming().next() {
            Some(Ok(mut stream)) => {
                let mut buf = [0u8; 4096];
                match stream.read(&mut buf) {
                    Ok(0) => continue,
                    Ok(n) => match str::from_utf8(&buf[..n]) {
                        Ok(request) => {
                            let mut splitted_request = request.split_whitespace();
                            let method = splitted_request.next();
                            let body = splitted_request.next();
                            match method {
                                Some("put") => {
                                    if let Some(x) = body {
                                        handle_put(x, file_path);
                                    } else {
                                        println!("Cannot handle put command");
                                    }
                                }
                                Some("get") => {
                                    if let Some(x) = body {
                                        handle_get(x, file_path);
                                    } else {
                                        println!("Cannot handle get command");
                                    }
                                }
                                Some("maxoffset") => {
                                    handle_maxoffset(file_path);
                                }
                                _ => println!("Cannot handle command"),
                            }
                        }
                        Err(_) => continue,
                    },
                    Err(_) => continue,
                }
            }
            Some(Err(_)) => continue,
            None => continue,
        }
    }
}

fn handle_put(command: &str, file_path: &str) {
    println!("Method: put; Data: {}", command);
    let mut buffer = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();

    buffer
        .write_all(format!("{}\n", command).as_bytes())
        .unwrap();
}

fn handle_get(command: &str, file_path: &str) {
    println!("Method: get; Data: {}", command);
    let offset = command.parse::<usize>().unwrap();
    let mut buffer = File::open(file_path).unwrap();
    let mut buf = String::new();
    buffer.read_to_string(&mut buf).unwrap();
    let line = buf.lines().nth(offset).unwrap();
    println!("{}", line);
}

fn handle_maxoffset(file_path: &str) {
    println!("Method: maxoffset;");
    let mut buffer = File::open(file_path).unwrap();
    let mut buf = String::new();
    buffer.read_to_string(&mut buf).unwrap();

    let lines_count = buf.lines().count();
    if lines_count == 0 {
        println!("No messages found");
        return;
    }

    let max_offset = lines_count - 1;
    println!("{}", max_offset);
}
