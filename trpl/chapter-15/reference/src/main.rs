use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // &self.0 accesses the first value in a tuple struct
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum ListRefCnt {
    Value(i32, Rc<ListRefCnt>),
    Null,
}

use crate::ListRefCnt::{Null, Value};
use std::rc::Rc;

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let z = 5;
    let t = Box::new(z);
    assert_eq!(5, z);
    assert_eq!(5, *t);

    let a = 5;
    let b = MyBox::new(a);
    assert_eq!(5, a);
    assert_eq!(5, *b);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // Error, cannot call it explicitly
    // c.drop();

    // but can use std::mem::drop in the prelude, it makes sure that
    // the object is dropped only once to avoid double-free problem
    drop(c);

    let e = Rc::new(Value(5, Rc::new(Value(10, Rc::new(Null)))));
    println!("counter after creating e = {}", Rc::strong_count(&e));
    // Rc::clone doesn't make a deep copy of all the data like most
    // types' implementations of clone do. The call to Rc::clone only
    // increments the reference count, which doesn't take much time.
    let f = Value(3, Rc::clone(&e));
    println!("counter after creating f = {}", Rc::strong_count(&e));
    {
        let g = Value(4, Rc::clone(&e));
        println!("counter after creating g = {}", Rc::strong_count(&e));
    }
    println!("counter after g goes out of scope = {}", Rc::strong_count(&e));
}
