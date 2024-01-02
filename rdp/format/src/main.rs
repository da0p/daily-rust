// Idiom 2: Use format to build up strings rather than using push and push_str
// methods on a mutable String, or using its + operator.

fn say_hello_1(name: &str) -> String {
    // We could construct the result string manually.
    let mut result = "Hello ".to_owned();
    result.push_str(name);
    result.push('!');

    result
}

fn say_hello_2(name: &str) -> String {
    format!("Hello {}!", name)
}

fn main() {
    println!("{}", say_hello_1("Phuong"));
    println!("{}", say_hello_2("Dao"));
}
