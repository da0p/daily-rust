use std::{path::PathBuf, time::Duration};

// Many types in Rust have a constructor. However, this is specific to
// the type; Rust cannot abstract over "everything that has a new() method".
// To allow this, the Default trait was conceived, which can be used with
// containers and other generic types(e.g. see Option::unwrap_or_default()).
// Notably, some containers already implement it where applicable.

// Rust supports default constructors with the Default trait
pub struct Second {
    value: u64,
}

impl Second {
    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Default for Second {
    fn default() -> Self {
        Self { value: 0 }
    }
}

// Default can also be derived if all types of all fields implement Default
#[derive(Default)]
pub struct Minute {
    value: u64,
}

impl Minute {
    /// Returns the value in minutes.
    pub fn value(&self) -> u64 {
        self.value
    }
}

#[derive(Default, Debug, PartialEq)]
struct MyConfiguration {
    // Option defaults to None
    output: Option<PathBuf>,
    // Vecs default to empty vector
    search_path: Vec<PathBuf>,
    // Duration defaults to zero time
    timeout: Duration,
    // bool defaults to false
    check: bool,
}

impl MyConfiguration{
    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = Duration::from_secs(timeout);
    }
}

fn main() {
    // construct a new instance with default values
    let mut conf = MyConfiguration::default();
    // do something with conf here
    conf.check = true;
    println!("conf = {:#?}", conf);
    // partial initialization with default values, creates the same instance
    let conf1 = MyConfiguration{
        check: true,
        ..Default::default()
    };
    assert_eq!(conf, conf1);
}
