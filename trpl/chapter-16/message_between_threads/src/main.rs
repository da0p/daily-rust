use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // The first element is the sending end - the transmitter
    // The second element is the receiving end - the receiver
    let (tx, rx) = mpsc::channel();

    // Let's create multiple producers
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            // send() will return a Result<T, E> 
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // recv() is a blocking method waiting for a value
    //let received = rx.recv().unwrap();

    // can use rx as an iterator
    for received in rx {
        println!("Got: {}", received);
    }
}
