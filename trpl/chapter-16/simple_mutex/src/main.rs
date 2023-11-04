use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // lock() will block until acquiring the lock. it would fail
        // if the thread currently holding the lock panics, then unwrap()
        // will panic on this thread
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}
