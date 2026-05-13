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
        let method = splitted_input.next().unwrap();
        let body = splitted_input.next();
        match method {
            "put" => {
                handle_put(body.unwrap(), file_path);
            }
            "get" => {
                handle_get(body.unwrap(), file_path);
            }
            "maxoffset" => {
                handle_maxoffset(file_path);
            }
            _ => panic!("Command unknown"),
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
    let line = buf.lines().count();
    let offset = line - 1;
    println!("{}", offset);
}
