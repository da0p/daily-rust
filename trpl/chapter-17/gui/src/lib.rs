pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Vec<Box<dyn Draw>> means any type inside a Box that
    // implements the Draw trait
    // Here we use trait objects. A trait object points to both
    // an instance of a type implementing our specified trait and
    // a table used to look up trait methods on the type at runtime
    //. We create a trait object by specifying some sort of pointer,
    // such as a & reference or a Box<T> smart pointer, then the 
    // dyn key word, and then specifying the relevant trait.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button!");
    }
}
