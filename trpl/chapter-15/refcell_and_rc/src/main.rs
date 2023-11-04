// Since Rc<T> allows multiple owners of immutable data, in contrast, RefCell<T>
// allows a single owner of mutable data, we can combine them to have the
// possibility to have multiple owners to mutable data.
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a after = {:?}", a);

    *value.borrow_mut() += 10;

    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
