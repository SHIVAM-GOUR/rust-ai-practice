// code goes here

struct User {
    name: String,
    age: i8,
    cars: i8,
}

impl User {
    // constructor
    fn new(name: String, age: i8, cars: i8) -> User {
        User { name, age, cars }
    }

    // method
    fn printUser(&self) {
        println!("name - {}", self.name);
        println!("age - {}", self.age);
        println!("cars - {}", self.cars);
    }
}

fn main() {
    let s = User::new(String::from("shivam gour"), 27, 1);

    s.printUser();
}
