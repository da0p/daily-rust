use std::fmt::Display;

// trait bound is used to constraint a generic type
fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

// similar to trait bounds, an impl Trait syntax can be used in function
// arguments and return values
fn get_x(name: impl Display) -> impl Display {
    format!("Hello {name}")
}

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");

    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");

    let x = get_x("foo");
    println!("{x}");
}
