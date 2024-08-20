use std::fmt::Debug;

struct Container<T> {
    value: T,
}

impl<T: Debug> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }

    fn debug_info(&self) {
        println!("{:?}", self.value);
    }
}

fn main() {
    let int_container = Container::new(42);
    int_container.debug_info(); // Output: 42

    let str_container = Container::new("Hello, world!");
    str_container.debug_info(); // Output: "Hello, world!"
}
