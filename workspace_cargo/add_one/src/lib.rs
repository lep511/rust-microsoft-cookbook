use rand::Rng;

pub fn add_one(x: i32) -> i32 {
    // Generate a random number between 10 and 50
    let random_number = rand::thread_rng().gen_range(10..=50);
    x + random_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_one() {
        let result = add_one(5);
        assert_eq!(result, 5);
    }
}
