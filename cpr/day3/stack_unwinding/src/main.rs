use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        "No problem here!"
    });
    println!("{result:?}");

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    println!("{result:?}");
}
