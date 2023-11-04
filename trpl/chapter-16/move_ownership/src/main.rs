use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // move ownership of v into thread to avoid any race condition
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
