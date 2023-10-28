#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Use dot notation to access members
    println!("user1.email = {0}", user1.email);
    // Debug trait
    println!("{:?}", user1);

    // struct doesn't allow to use mut for each member
    // only mutable for the whole struct is possible
    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");
    println!("user2.email = {0}", user2.email);

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{:?}", user3);

    let user4 = User {
        email: String::from("another@example.com"),
        // copy the rest from user1
        ..user2
    };
    println!("{:?}", user4);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );

    // print pretty
    println!("{:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
