// 1. FnOnce applies to closures that can be called once. All closures
//    implement at least this trait, because all closures can be called.
//    A closure that moves captured values out of its body will only
//    implement FnOnce and none of the other Fn traits, because it can
//    only be called once.
// 2. FnMut applies to closures that don't move captured values out of
//    their body, but that might mutate the captured values. These closures
//    can be called more than once.
// 3. Fn applies to closures that don't move captured values out of their
//    body and that don't mutate captured values, as well as closures that
//    capture nothing from their environment. These closures can be called
//    more than once without mutating their environment, which is important
//    in cases such as calling a closure multiple times concurrently.

use core::num;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {width: 10, height: 1},
        Rectangle {width: 3,  height: 5},
        Rectangle {width: 7, height: 12},
    ];

    // FnMut
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // let mut sort_operations = vec![];
    //let value = String::from("by key called");
    // Error!!! The following closure should be FnOnce since it
    // moves values out of the environment, but the definition
    // is FnMut
    //
    // list.sort_by_key(|r| {
    //    sort_operations.push(value);
    //    r.width
    // });
    // println!("{:#?}", list);
    
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let sum = iterator_sum();
    println!("sum = {sum}");

    let map_sum = map_sum();
    println!("map_sum = {:#?}", map_sum);
}

fn iterator_sum() -> i32 {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // sum() takes onwnerhip of the iterator, so we can't use v1_iter
    // after using sum()
    let total: i32 = v1_iter.sum();

    total
}

fn map_sum() -> Vec<i32> {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2 = v1.iter().map(|x| x + 1).collect();

    v2
}