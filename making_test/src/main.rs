use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num.value);
    let sum_num = add(num.value, 10);
    println!("My sum is {:?}", sum_num);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(3));
    }

    #[test]
    fn test_from() {
        let num = Number::from(30);
        assert_eq!(num.value, 30);
    }

    #[test]
    #[should_panic]
    fn test_from_panic() {
        let num = Number::from(30);
        assert_eq!(num.value, 31);
    }

    #[test]
    #[ignore = "not yet reviewed by the Q.A. team"]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4)
    }
}

#[cfg(test)]
mod add_function_tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-2, -2), -4);
        assert_eq!(add(-2, 2), 0);
    }
}
