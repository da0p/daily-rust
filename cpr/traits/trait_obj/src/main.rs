struct Dog {
    name: String,
    age: i8,
}

struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!(
            "Woof, my name is {}! I am {} years old!",
            self.name, self.age
        )
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        format!("Miau! I am cat with {} lives", self.lives)
    }
}

fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat { lives: 9 }),
        Box::new(Dog {
            name: String::from("Fido"),
            age: 5,
        }),
    ];
    for pet in pets {
        println!("Hello, who are you? {}", pet.talk());
    }
}
