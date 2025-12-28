unsafe extern "C" {
    unsafe fn abs(input: i32) -> i32;
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x012345usize;
    let _r = address as *mut i32;

    //    let _values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
