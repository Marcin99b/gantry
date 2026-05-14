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
                println!("Connection from: {}", stream.peer_addr().unwrap().ip());
                let mut buf = [0u8; 4096];
                match stream.read(&mut buf) {
                    Ok(0) => continue,
                    Ok(n) => match str::from_utf8(&buf[..n]) {
                        Ok(request) => {
                            println!("Request: {}", request);
                            let response = handle_request(request, file_path);
                            println!("Response: {}", response);
                            let bytes = response.as_bytes();
                            stream.write_all(bytes).unwrap();
                        }
                        Err(_) => continue,
                    },
                    Err(_) => continue,
                }
                println!();
            }
            Some(Err(_)) => continue,
            None => continue,
        }
    }
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
                "Cannot handle put command".to_string()
            }
        }
        Some("get") => {
            if let Some(x) = body {
                handle_get(x, file_path)
            } else {
                "Cannot handle get command".to_string()
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
        return "No messages found".to_string();
    }

    let max_offset = lines_count - 1;
    max_offset.to_string()
}
