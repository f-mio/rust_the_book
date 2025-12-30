pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            //予想値は1から100の間でなければなりませんが、{}でした。
            panic!("Guess value must be between 1 and 100, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value)
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
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail.")
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8, height: 7
        };
        let smaller = Rectangle {
            width: 5, height: 1
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 8, height: 7
        };
        let smaller = Rectangle {
            width: 5, height: 1
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic]
    fn greater_then_100_0() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_then_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four."))
        }
    }

}




