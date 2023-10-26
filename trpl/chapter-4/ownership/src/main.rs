fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2 = {s2}");

    // Let's make a copy
    let s3 = s2.clone();

    println!("s2 = {s2}, s3 = {s3}");

    let s = String::from("world");

    // s's value moves into the function and no longer valid here
    takes_ownership(s);

    // x comes into scope
    let x = 5;

    // x would move into the function, but i32 is Copy, so it's okay
    // to still use x afterward
    makes_copy(x);

    // gives_ownership moves its return value into s4;
    let s4 = gives_ownership();
    println!("s4 = {s4}");

    let s5 = String::from("I'm D");

    // s5 is moved into
    // then the function's return value moves the value into s6
    let s6 = takes_and_gives_back(s5);
    println!("s6 = {s6}");

    // return multiple values
    let s7 = String::from("hello world");
    let (s8, len) = calculate_length(s7);
    println!("(s8, len) = ({s8}, {len})");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // some_string goes out of scope and `drop` is called. The
    // backing memory is freed
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
    // some_integer goes out of scope, nothing special happens.
}

fn gives_ownership() -> String {
    // gives_onwnership will move its return value into the
    // function that calls it
    let some_string = String::from("yours");

    // some_string is returned and moves out to the calling
    // function
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}