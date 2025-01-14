use std::io::stdin;

#[derive(Debug)]
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
        println!("{}", self.greeting);
    }
}

fn get_name() -> String {
    let mut name = String::new();

    stdin().read_line(&mut name).expect("Failed to read line");

    name.trim().to_lowercase()
}

fn main() {

    let mut visitor_list = vec![
        Visitor::new("joe", "Coffee is ready for you"),
        Visitor::new("fred", "Dreaded Fred"),
        Visitor::new("steve", "I am peeved at steve"),
    ];

    loop {
        println!("What is your name? Enter blank to stop");
        let name = get_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the list\nWould you like to be added?(y/n)", name);
                    let mut answer = String::new();
                    stdin().read_line(&mut answer).expect("Failed");

                    if answer.to_lowercase().starts_with("y") {
                        println!("Great a new friend! How shall I greet you?");
                        let mut new_greeting = String::new();
                        stdin().read_line(&mut new_greeting).expect("Failed");
                        visitor_list.push(Visitor::new(&name, new_greeting.trim()));
                    } else {
                        println!("Fine I have enough friends");
                    }
                }
            }
        }
    }
    println!("Final vector of visitors:\n{:#?}", visitor_list);


}
