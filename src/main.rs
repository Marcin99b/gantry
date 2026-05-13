use std::{
    fs::{File, OpenOptions},
    io::{Read, Write, stdin},
};

//struct Message {}

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut splitted_input = input.split_whitespace();
        let method = splitted_input.next().unwrap();
        let body = splitted_input.next().unwrap();
        match method {
            "put" => {
                put(body);
            }
            "get" => {
                get(body);
            }
            _ => panic!("Command unknown"),
        }
    }
}

fn put(command: &str) {
    println!("Method: put; Data: {}", command);
    let mut buffer = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data.txt")
        .unwrap();

    buffer
        .write_all(format!("{}\n", command).as_bytes())
        .unwrap();
}

fn get(command: &str) {
    println!("Method: get; Data: {}", command);
    let offset = command.parse::<usize>().unwrap();
    let mut buffer = File::open("data.txt").unwrap();
    let mut buf = String::new();
    buffer.read_to_string(&mut buf).unwrap();
    let line = buf.lines().nth(offset).unwrap();
    println!("{}", line);
}
