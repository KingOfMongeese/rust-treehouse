use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String},
    Refuse,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: i8
}

impl Visitor {
    fn new(name: &str, greeting: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
       
       match &self.action {
        VisitorAction::Accept => println!("{}", self.greeting),

        VisitorAction::AcceptWithNote { note } => {
            println!("{}", self.greeting);
            println!("Note: \n{}", note);
        }

        VisitorAction::Refuse => println!("You are not allowed in."),

       }

       if self.age < 21 {
            println!("We cannot serve you alcohol");
       }
    }
}

fn get_name() -> String {
    let mut name = String::new();

    stdin().read_line(&mut name).expect("Failed to read line");

    name.trim().to_lowercase()
}

fn main() {

    let mut visitor_list = vec![
        Visitor::new("joe", "Coffee is ready for you", VisitorAction::Accept, 45),
        Visitor::new("fred", "Dreaded Fred", VisitorAction::AcceptWithNote { 
            note: String::from("Fred is dreaded, he needs a haircut") }, 15),
        Visitor::new("steve", "I am peeved at steve", VisitorAction::Refuse, 30),
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

                        let mut action = VisitorAction::Accept;
                        
                        // get greetting
                        println!("Great a new friend! How shall I greet you?");
                        let mut new_greeting = String::new();
                        stdin().read_line(&mut new_greeting).expect("Failed");
                        
                        // get new note
                        println!("Do you need a note?(y/n)");
                        let mut needs_note = String::new();
                        stdin().read_line(&mut needs_note).expect("Failed");
                        if needs_note.to_lowercase().starts_with("y") {
                            println!("Enter the note:");
                            let mut note = String::new();
                            stdin().read_line(&mut note).expect("Failed");
                            action = VisitorAction::AcceptWithNote { note: String::from(note.trim()) }

                        }

                        // get age
                        println!("Enter your age:");
                        let mut age = String::new();
                        stdin().read_line(&mut age).expect("Failed");

                        visitor_list.push(Visitor::new(&name, new_greeting.trim(), action, age.trim().parse().unwrap()));
                    } else {
                        println!("Fine I have enough friends");
                    }
                }
            }
        }
        println!("---------------------------------------------")
    }
    println!("Final vector of visitors:\n{:#?}", visitor_list);


}
