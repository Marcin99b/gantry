use std::io::stdin;

//struct Message {}

fn main() {
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

fn put(command: &str) {
    println!("put: {}", command);
}

fn get(command: &str) {
    println!("get: {}", command);
}
