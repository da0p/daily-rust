#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// We can define methods on enum
impl Message {
    fn call(&self) {
        println!("This is a CALL!");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // remember that matchs are exauhstive, you must cover all
    // possible cases, otherwise it's considered a bug.
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn roll_dice(number: u8) {
    match number {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn add_fancy_hat() {
    println!("Add a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Remove fancy hat!");
}

fn move_player(num_spaces: u8) {
    println!("Move player {} steps!", num_spaces);
}

fn main() {
    let home = IpAddr::V4((String::from("127.0.0.1")));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("home = {:#?}", home);
    println!("loopback = {:#?}", loopback);

    let message = Message::Write((String::from("Hello")));
    message.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    value_in_cents(Coin::Penny);

    value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    let six = plus_one(five);
    println!("six = {}", six.unwrap());
    let none = plus_one(None);

    roll_dice(9);
    roll_dice(3);
    roll_dice(7);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Nickel;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("Number of not quarter-coin = {}", count);
}
