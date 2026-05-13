use std::{
    fs::{File, OpenOptions},
    io::{Read, Write, stdin},
};

//struct Message {}

fn main() {
    let file_path = "data.txt";
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut splitted_input = input.split_whitespace();
        let method = splitted_input.next();
        let body = splitted_input.next();
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
