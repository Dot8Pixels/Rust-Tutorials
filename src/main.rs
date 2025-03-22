trait Greet {
    fn say_hello(&self);
}

trait Farewell {
    fn say_goodbye(&self);
}

fn conversation<T: Greet + Farewell>(t: T) {
    t.say_hello();
    t.say_goodbye();
}

struct Person;

impl Greet for Person {
    fn say_hello(&self) {
        println!("Hello!");
    }
}

impl Farewell for Person {
    fn say_goodbye(&self) {
        println!("Goodbye!");
    }
}

fn main() {
    let person = Person;
    conversation(person);
}
