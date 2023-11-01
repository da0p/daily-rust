use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1,
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2,
    );

    // Closures can capture values from their environment in three ways
    // - borrowing immutably
    // - borrowing mutably
    // - taking ownership

    // 1st: Borrowing immutably
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);

    only_borrows();
    println!("After calling closure: {:?}", list);

    // 2nd: Borrowing mutably
    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list2);

    let mut borrows_mutably = || list2.push(7);
    // Error here because list2 is borrowed mutably, so we can't
    // borrow it immutably anymore, we need to use it first
    //println!("Before calling closure: {:?}", list2);

    borrows_mutably();

    // Error the same as above
    // println!("After calling closure: {:?}", list2);
    borrows_mutably();
    println!("After calling closure: {:?}", list2);

    // 3rd way: use move to take onwnership
    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list3);

    thread::spawn(move || println!("From thread: {:?}", list3))
        .join()
        .unwrap();
}
