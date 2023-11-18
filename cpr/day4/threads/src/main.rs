use std::thread;
use std::time::Duration;

// fn foo() {
//    let s = String::from("Hello");
//    thread::spawn(|| {
//        println!("Length: {}", s.len());
//    });
// }

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }

    // Thread cannot borrow from their environment, the following
    // line will create an error
    // foo();

    // However, if we use scoped thread, it's possible
    let s = String::from("Hello");
    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Length: {}", s.len());
        });
    });
}
