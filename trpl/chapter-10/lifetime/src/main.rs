use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

impl<'a> ImportantExcerpt<'a> {
    // no need to annotate the lifetime of the reference to self
    // because of the first elision rule.
    fn level(&self) -> i32 {
        3
    }

    // Two input lifetimes, first rule applies gives lifetime
    // to both &self and &str, then third rule applies since
    // one of the parameters is &self, the return type gets 
    // the lifetime of &self.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result2 = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result2);
    }

    let string5 = String::from("long string is long");
    let result3;
    {
        let string6 = String::from("xyz");
        result3 = longest(string5.as_str(), string6.as_str());
    }
    // This is gonna be an error
    //println!("The longest string is {}", result3);

    let novel = String::from("Call me Ishmaekl. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetime means that the affected reference can live
    // for the entire duration of the program.
    let _s: &'static str = "I have a static lifetime.";
}

// The function signature says that for some lifetime 'a, the
// function takes two parameters, both of which are string slices
// that live at least as long as lifetime 'a. It also tells that
// the string slice returned from the function will live at least
// as long as lifetime 'a.
// In practice, it means that the lifetime of the reference returned
// by the longest function is the same as the smaller of the lifetimes
// of the values referred to by the function arguments.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
