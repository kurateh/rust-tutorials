#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 60,
    };
    let rect3 = Rectangle {
        width: 20,
        height: 20,
    };
    let rect4 = Rectangle::square(40);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!(
        "Can rect1 hold rect3? {}",
        Rectangle::can_hold(&rect1, &rect3)
    );
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));
}
