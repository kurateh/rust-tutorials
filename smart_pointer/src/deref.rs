use std::ops::Deref;

// 15.2
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub fn deref_tutorial() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // &MyBox<String> => &String => &str

    // If there doesn't exit Deref coercion:
    hello(&(*m)[..]);
}
