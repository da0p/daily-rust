
fn main() {
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    // It will return Option<&i8> since iter() does not take
    // ownership
    let v0: Option<&i8> = iter.next();
    println!("v0: {v0:?}");

    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = v.into_iter();

    // Instead, into_iter() take ownership
    let v0: Option<String> = iter.next();
    println!("v0: {v0:?}");

    let v1: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v1 {
        println!("word: {word}");
    }

    for word in v1 {
        println!("word: {word}");
    }
}
