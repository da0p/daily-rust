// Option can be viewed as a container that contains either zero or
// one element. In particular, it implements the IntoIterator trait, 
// and as such can be used with generic code that needs such a type.

fn main() {

    let turing = Some("Turing");
    let mut logicians = vec!["Curry", "Kleene", "Markov"];

    logicians.extend(turing);

    // equivalent
    if let Some(turing_inner) = turing {
        logicians.push(turing_inner);
    }

    // chain() can be used to put one Option to the end of an exisitng iterator
    for logician in logicians.iter().chain(turing.iter()) {
        println!("{} is a logician", logician);
    }
}
