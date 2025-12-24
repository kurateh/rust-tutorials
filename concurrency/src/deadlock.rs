use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[allow(unused)]
pub fn deadlock_tutorial() {
    let num1 = Arc::new(Mutex::new(1));
    let num2 = Arc::new(Mutex::new(1));
    let handle1;
    let handle2;

    {
        let num1 = num1.clone();
        let num2 = num2.clone();
        handle1 = thread::spawn(move || {
            let _guard1 = num1.lock().unwrap();
            thread::sleep(Duration::from_secs(1));

            let _guard2 = num2.lock().unwrap();
        });
    }

    {
        let num1 = num1.clone();
        let num2 = num2.clone();
        handle2 = thread::spawn(move || {
            let _guard2 = num2.lock().unwrap();
            thread::sleep(Duration::from_secs(1));

            let _guard1 = num1.lock().unwrap();
        });
    }

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("deadlock!");
}
