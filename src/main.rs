use std::io::stdin;

//struct Message {}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    println!("{}", input);
}
