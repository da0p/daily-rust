use std::mem;

enum MyEnum {
    A { name: String, x: u8 },
    B { name: String }
}

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        // This takes out our `name` and puts in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will)
        // be assigned to `*e`
        *e = MyEnum::B { name: mem::take(name) }
    }
}

enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D
}

fn swizzle(e: &mut MultiVariateEnum) {
    use MultiVariateEnum::*;
    *e = match e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { name } => B { name: mem::take(name) },
        B { name } => A { name: mem::take(name) },
        C => D,
        D => C,
    }
}

fn main() {
    let mut e = MyEnum::A {name: "Hello".to_string(), x: 10 };
    a_to_b(&mut e);

    let mut f = MultiVariateEnum::A{name: "Phuong".to_string()};
    swizzle(&mut f);
}
