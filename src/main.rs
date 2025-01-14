use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        print!("{}", self.greeting);
    }
}

fn get_name() -> String {
    let mut name = String::new();

    stdin().read_line(&mut name).expect("Failed to read line");

    name.trim().to_lowercase()
}

fn main() {
    println!("Hello, whats your name?\n>");
    let name = get_name();

    let visitor_list = [
        Visitor::new("joe", "Coffee is ready for you"),
        Visitor::new("fred", "Dreaded Fred"),
        Visitor::new("steve", "I am peeved at steve"),
    ];

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list. Security will show you out...") 
    }
}
