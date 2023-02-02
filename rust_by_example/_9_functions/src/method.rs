struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

#[test]
pub fn example() {
    let peter = Person::new(String::from("Peter"), 27);
    peter.say_hello();

    let a = &&&&7;
    let b = a.to_owned();
    let c = b.to_owned();
}
