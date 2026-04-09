#[allow(dead_code)]
pub fn valid_palindrome(s: String) -> bool {
    let str: String = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    let bytes: Vec<u8> = str.bytes().collect();

    for (i, byte) in bytes.iter().rev().enumerate() {
        if *byte != bytes[i] {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let s: String = String::from("A man, a plan, a canal: Panama");
        assert_eq!(valid_palindrome(s), true);
    }

    #[test]
    fn case_2() {
        let s: String = String::from("race a car");
        assert_eq!(valid_palindrome(s), false);
    }

    #[test]
    fn case_3() {
        let s: String = String::from(" ");
        assert_eq!(valid_palindrome(s), true);
    }
}
