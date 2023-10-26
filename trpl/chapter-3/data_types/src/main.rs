fn main() {
    let x = 2.0; // f64
    println!("x = {x}");

    let y: f32 = 3.0; // f32
    println!("y = {y}");

    // addition
    let sum = 5 + 10;
    println!("sum = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product = {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient = {quotient}");

    let truncated = -5 / 3;
    println!("truncated = {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder = {remainder}");

    let t = true;
    println!("t = {t}");

    let f: bool = false;
    println!("f = {f}");

    let tup_1:(i32, f64, u8) = (500, 6.4, 1);
    println!("tup_1 = {:?}", tup_1);

    let tup_2:(i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup_2.0;
    println!("tup_2.x = {five_hundred}");

    let six_point_four = tup_2.1;
    println!("tup_2.y = {six_point_four}");

    let array_1 = [1, 2, 3, 4, 5];
    println!("array_1 = {:?}", array_1);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("months = {:?}", months);

    let array_2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array_2 = {:?}", array_2);

    let array_3 = [3;5];
    println!("array_3 = {:?}", array_3);

    println!("months[0] = {months[0]}");
}