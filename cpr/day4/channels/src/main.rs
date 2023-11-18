use std::sync::mpsc;

fn main() {
    // Rust channels have two parts: a Sender<T> and a Receiver<T>.
    // The two parts are connected via the channel, but we see only
    // the end-points.
    let (tx, rx) = mpsc::channel();

    tx.send(10).unwrap();
    tx.send(20).unwrap();

    println!("Received: {:?}", rx.recv());
    println!("Received: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("Received: {:?}", rx.recv());
}
