use advanced_trait::{Counter, Iterator};
use std::ops::Add;
use std::fmt;

// This means that OutlinePrint trait will work only for types
// that also implement Display and provide the functionality
// that OUtlinePrint needs.
// It's similar to adding a trait bound to the trait.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*". repeat(len + 4));
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

#[derive(Debug)]
pub struct Millimeters {
    pub distance: u32,
}

#[derive(Debug)]
pub struct Meters {
    pub distance: u32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters{distance: self.distance + (other.distance * 1000)}
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// NewType Pattern
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let mut counter = Counter { counter: 10 };

    println!("next = {}", counter.next().unwrap());

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let point = Point {x: 1, y: 3};
    point.outline_print();

    let first = Millimeters{distance: 10};
    let second = Meters{distance: 20};
    println!("first + second = {:#?}", first + second);

    let person = Human;
    // call it explicitly to get the expected version
    // This is called fully qualified syntax
    Pilot::fly(&person);
    Wizard::fly(&person);

    // otherwise will call the version in Human
    person.fly();

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
