// 15.3
#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
#[allow(unused_variables)]
pub fn drop_tutorial() {
    let early = CustomSmartPointer {
        data: String::from("early dropped"),
    };
    let automatically = CustomSmartPointer {
        data: String::from("automatically dropped"),
    };

    println!("CustomSmartPointer created");
    drop(early);
    println!("CustomSmartPointer c dropped before the end of main");
}
