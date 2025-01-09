use std::io::stdin;

fn main() {
    println!("Hello, whats your name?\n>");
    let mut name = String::new();

    stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello {}!", name.trim());
}
