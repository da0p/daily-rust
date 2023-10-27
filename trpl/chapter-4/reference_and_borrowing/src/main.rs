fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");

    change(&mut s2);

    println!("some_string = {s2}");

    let s3 = String::from("hello world");

    let word = first_word(&s3);

    // This will create a compile-time error!
    // s3.clear();
    println!("the first word is: {}", word);

    let my_string = String::from("Hello World!");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("word = {word}");
    let word = first_word(&my_string[..]);
    println!("word = {word}");
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("word = {word}");

    let my_string_literal = "hello world again";
    //`first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("word = {word}");
    let word = first_word(&my_string_literal[..]);
    println!("word = {word}");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("word = {word}");

    let array_1 = [1, 2, 3, 4, 5];

    let slice_1 = &array_1[1..3];

    assert_eq!(slice_1, &[2, 3]);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


// Accept a reference to a string slice, and return a string slice
fn first_word(s: &str) -> &str {
   let bytes = s.as_bytes();

   for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
        return &s[0..i];
    }
   } 
   &s[..]
}