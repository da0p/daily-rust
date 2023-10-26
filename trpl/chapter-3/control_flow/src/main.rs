fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // rust is very picky, you must specify the correct type, no implicit
    // conversion to bool
    if number != 0 {
        println!("number was something other than zero");
    }

    let number_2 = 6;

    if number_2 % 4 == 0 {
        println!("number is divisible by 4");
    } else if number_2 % 3 == 0 {
        println!("number is divisible by 3");
    } else if number_2 % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if is an expression, we can use it on the right side of a let statement
    // to assign the outcome to a variable
    let condition = true;
    let number_3 = if condition { 5 } else { 6 };

    println!("The value of number_3 is: {number_3}");

    // returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // break the inner loop
                break;
            }
            if count == 2 {
                // break the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    // or
    println!("\n\n");
    for element in a {
        println!("The value is: {element}");
    }

    println!("\n\n");
    // print in reverse with range
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
