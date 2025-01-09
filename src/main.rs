use std::io::stdin;

fn get_name() -> String {
    let mut name = String::new();

    stdin().read_line(&mut name).expect("Failed to read line");

    name.trim().to_lowercase()
}

fn main() {
    println!("Hello, whats your name?\n>");
    let name = get_name();

    println!("Hello {}!", name);
}
