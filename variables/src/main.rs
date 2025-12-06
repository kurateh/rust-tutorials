use std::io;

fn main() {
    let mut input: String;

    loop {
        input = "".to_string();
        println!("Which Fibonacci number do you want to know? If you want to quit, input \"quit\"");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input = input.trim();

        if input == "quit" {
            break;
        }

        let input: usize = match input.parse() {
            Ok(0) => continue,
            Ok(num) => num,
            Err(_) => continue,
        };

        let ordinal_suffix = match input {
            _ if input % 10 == 1 && input % 100 != 11 => "st",
            _ if input % 10 == 2 && input % 100 != 12 => "nd",
            _ if input % 10 == 3 && input % 100 != 13 => "rd",
            _ => "th",
        };

        println!(
            "The {input}{ordinal_suffix} Fibonacci number is {}\n",
            fibonacci(input).format_with_commas()
        );
    }
}

fn fibonacci(n: usize) -> u128 {
    let mut arr: [u128; 3] = [1, 1, 2];
    if n <= 3 {
        return arr[n - 1];
    }

    for _ in 4..=n {
        arr[0] = arr[1];
        arr[1] = arr[2];
        match arr[0].checked_add(arr[1]) {
            Some(val) => {
                arr[2] = val;
            }
            None => return 0,
        }
    }

    arr[2]
}

trait CommaFormatted {
    fn format_with_commas(&self) -> String;
}

impl CommaFormatted for u128 {
    fn format_with_commas(&self) -> String {
        let num_str = self.to_string();
        let mut result = String::new();

        for (i, c) in num_str.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.push(',');
            }
            result.push(c);
        }

        result.chars().rev().collect()
    }
}
