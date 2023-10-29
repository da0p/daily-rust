use std::fs::{self, File};
// ErrorKind provides variants representing the different kinds of errors
// that might result from an io operation.
use std::io::{self, ErrorKind, Read};

fn main() {
    let open_file_result = File::open("hello.txt");

    let _greeting_file = match open_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // another way, if the file doesn't exist, unwrap will call panic!
    let _greeting_file_2 = File::open("hello.txt").unwrap();

    // in order to convey a better message, this is the production-quality approach
    let _greeting_file_3 =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // propagating errors
    let _greeting_file_4 = read_username_from_file_1();

    let _greeting_file_5 = read_username_from_file_2();

    let _greeting_file_6 = read_username_from_file_3();
}

fn read_username_from_file_1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    // ? placed after a Result value is defined to work in almost the
    // same way as the match expressions we defined tohandle the Result
    // values. If the value of the Result is an Ok, the value inside the
    // Ok will get returned from this expression, and the program will
    // continue. If the value is an Err, the Err wil be returned from the
    // whole function as if we had used the return keyword sot he error
    // value gets propagated to the calling code.
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    // using standard library to make it even shorter
    fs::read_to_string("hello.txt")
}
