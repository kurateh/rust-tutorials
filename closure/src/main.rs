use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operation_count: u32 = 0;
    list.sort_by_key(|r| {
        sort_operation_count += 1;
        r.width * r.height
    });
    println!("{:#?}, sorted in {sort_operation_count} operations", list);
}
