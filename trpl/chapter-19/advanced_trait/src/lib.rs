pub trait Iterator {
    // Associated types connect a type placeholder with a trait such that
    // the trait method definitions can use these placeholder types in
    // their signatures. The implementor of a trait will specify the
    // concrete type to be used instead of the placeholder type for the
    // particular implementation.
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {
    pub counter: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.counter + 1)
    }
}

