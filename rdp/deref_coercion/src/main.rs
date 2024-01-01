// Using a target of a deref coercion can increase the flexibility of your code
// when you are deciding which argument type to use for a function argument. 
// In this way, the function will accept more input types.
//
// This is not limited to slice-able or fat pointer types. In fact, you should
// always prefer using the borrowed type over borrowing the owned type. Such
// as &str over &String, &[T] over &Vec<T>, or &T over &Box<T>.
//
// Using borrowed types you can avoid layers of indirection for those instances
// where the owned type already provides a layer of indirection. For instance,
// a String has a layer of indirection, so a &String will have two layers of
// indirection. We can avoid this by using &str instead, and letting &String
// coerce to a &str whenever the function is invoked.

// If word is &String, then we can't use &'static str as an input
fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            }
            _ => vowel_count = 0
        }
    }
    false
}

fn main() {
    let ferris = "Ferrir".to_string();
    let curios = "Curios".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curios, three_vowels(&curios));
    println!("Ferris: {}", three_vowels("Ferris"));
    println!("Curious: {}", three_vowels("Curious"));

    let sentence_string = "Once upon a time, there was a friendly curious crab named Ferries".to_string();
    for word in sentence_string.split(' ') {
        if three_vowels(word) {
            println!("{} has three consecutive vowels!", word);
        }
    }
}
