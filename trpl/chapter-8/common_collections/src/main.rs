use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // a macro to create a vector
    let _v1 = vec![1, 2, 3];

    // Let's update a vector
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    let v3 = vec![1, 2, 3, 4, 5];

    // reading an element of vector
    let third: &i32 = &v3[2];
    println!("The third element is {third}");

    // we can use get method to get an Option
    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // if not using get, the program will panic
    let none: Option<&i32> = v3.get(100);
    match none {
        Some(none) => println!("The element is {none}"),
        None => println!("There is no element at this index"),
    }

    let mut v4 = vec![1, 2, 3, 4, 5];
    let _first = &v4[0];
    v4.push(6);
    // The next line won't work because first is holding an immutable
    // reference
    // println!("The first element is: {first}");

    // Iterating over a vector using immutable reference
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{i}");
    }

    // Iterating over a vector using mutable reference
    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        *i += 50;
    }
    println!("{:?}", v6);

    // Use enum to store multiple types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);

    // String topic
    // This is a string literal. It can be converted into String
    // using to_string, since string literal implements the
    // Display trait
    let data = "initial contents";

    let s1 = data.to_string();
    println!("{}", s1);
    let s2 = "initial contents".to_string();
    println!("{}", s2);

    // Let's update a string
    let mut s3 = String::from("foo");
    s3.push_str("bar");

    let mut s4 = String::from("foo");
    let s5 = "bar";
    // push_str doesn't take ownership
    s4.push_str(s5);
    println!("s4 is {s4}");
    println!("s5 is {s5}");

    // push method to append a single character
    let mut s7 = String::from("lo");
    s7.push('l');
    println!("s7 is {s7}");

    // concatenation with + operator and format! macro
    let s8 = String::from("Hello, ");
    let s9 = String::from("world!");
    // s8 will be moved here and can no longer be used
    // we need to use &s9 due to the signature of the method
    // since it's not &self, then this method will take ownership
    // of the first string
    // fn add(self, s: &str) -> String
    let s10 = s8 + &s9;
    println!("s10 is {s10}");

    // Let's concatenate multiple strings
    let s11 = String::from("tic");
    let s12 = String::from("tac");
    let s13 = String::from("toe");

    let s14 = s11 + "-" + &s12 + "-" + &s13;
    println!("s14 is {s14}");

    // we can format string by using format!() macro
    let s15 = String::from("tic");
    // format! use reference, so it doesn't take ownership of
    // the strings
    let s16 = format!("{s15}-{s12}-{s13}");
    println!("s16 is {s16}");

    // We can't access string by index, since it depends on the size
    // of the encoding type, and rust doesn't compile to prevent
    // any mistake
    // The best way to iterate over strings is to be explicit if you
    // want characters or bytes
    for c in "ab".chars() {
        println!("{c}");
    }

    for b in "ab".bytes() {
        println!("{b}");
    }

    // Let's explore hash maps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // overwriting
    scores.insert(String::from("Blue"), 20);

    // adding a key and value if a key is not present
    scores.entry(String::from("Yellow")).or_insert(100);
    println!("scores = {:?}", scores);
    scores.entry(String::from("Green")).or_insert(1000);
    println!("scores = {:?}", scores);

    // accessing a hash map
    let team_name = String::from("Blue");
    // we need to copy in order to avoid taking ownership
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score = {score}");

    // iterating over a hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    // For types that implement the Copy trait, like i32, the values are
    // copied into the hash map. For owned values like String, the values
    // will be moved and hash map will be the owner of those values
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // println!("field_name = {field_name}, field_value = {field_value}");

    // Update a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
