use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let v = Arc::new(vec![10, 20, 30]);
    let mut handles = Vec::new();
    for _ in 1..5 {
        let v = Arc::clone(&v);
        handles.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            println!("{thread_id:?}: {v:?}");
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("v: {v:?}");

    let v2 = Mutex::new(vec![10, 20, 30]);
    println!("v2: {:?}", v2.lock().unwrap());
    {
        let mut guard = v2.lock().unwrap();
        guard.push(40);
    }
    println!("v2: {:?}", v2.lock().unwrap());

    let v3 = Arc::new(Mutex::new(vec![10, 20, 30]));
    let v4 = v3.clone();
    let handle = thread::spawn(move || {
        v4.lock().unwrap().push(40);
    });

    v3.lock().unwrap().push(1000);
    handle.join().unwrap();
    println!("v3: {v3:?}");
}
