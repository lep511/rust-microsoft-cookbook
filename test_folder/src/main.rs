fn is_palindrome(word: &str) -> bool {
    // Convert the word to lowercase and remove non-alphanumeric characters.
    let word = word
        .chars()
        .filter_map(|c| if c.is_alphanumeric() { Some(c.to_lowercase().to_string()) } else { None })
        .collect::<String>();

    // Check if the word is the same forwards and backwards.
    word == word.chars().rev().collect::<String>()
}

fn main() {
    let word = "Racecar"; // Example word
    if is_palindrome(word) {
        println!("'{}' is a palindrome.", word);
    } else {
        println!("'{}' is not a palindrome.", word);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_true() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(is_palindrome("level"));
    }

    #[test]
    fn test_is_palindrome_false() {
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("world"));
        assert!(!is_palindrome("1234"));
    }
}