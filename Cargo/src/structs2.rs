struct Person{
    first_name: String,
    last_name: String,
}

impl Person {

    //contructer (construct person)
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name) // macro similar to println but doesnt actually print
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to Tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }

}

pub fn run() {
    let mut p = Person::new("Ajitesh","Panda");
    println!("Person: {}", p.full_name());
    let mut p1 = Person::new("Ajitesh","Panda");
    p1.set_last_name("Yash");
    println!("Person1: {}", p1.full_name());
    //println!("Person: {} {} ", p.first_name, p.last_name);
    println!("Person in Tuple: {:?}", p1.to_tuple()); // while printing the tuple container its important to use the debugging trait
}