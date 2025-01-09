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

    let mut is_on_list = false;
    let visitor_list = [
        Visitor::new("joe", "Coffee is ready for you"),
        Visitor::new("fred", "Dreaded Fred"),
        Visitor::new("steve", "I am peeved at steve"),
    ];

    for visitor in &visitor_list {
        if visitor.name == name {
            is_on_list = true;
            break;
        }
    }

    if is_on_list {
        println!("Hello {}!", name);
    } else {
        print!("You are not on the list");
    }
}
