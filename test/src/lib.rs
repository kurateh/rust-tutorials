#[allow(dead_code, unused)]
#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code, unused)]
impl Rectangle {
    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[allow(dead_code, unused)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub const LESS_THAN_1_MESSAGE: &str = "Guess value must be greater than or equal to 1";
    pub const GREATER_THAN_100_MESSAGE: &str = "Guess value must be less than or equal to 100";

    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("{}, got {}", Self::LESS_THAN_1_MESSAGE, value);
        }

        if value > 100 {
            panic!("{}, got {}", Self::GREATER_THAN_100_MESSAGE, value);
        }

        Guess { value }
    }
}

pub fn add_two(value: i32) -> i32 {
    value + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn explore() {
        assert_eq!(2 + 2, 4);
        assert_eq!("asdf".to_string(), "asdf");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(larger.can_hold(&smaller));
        assert_eq!(larger, larger);
    }

    #[test]
    #[should_panic(expected = "must be less")]
    fn greater_than_100() {
        Guess::new(101);
    }
}
