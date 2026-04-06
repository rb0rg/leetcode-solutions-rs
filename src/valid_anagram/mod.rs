#[allow(dead_code)]
pub fn valid_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut count: [i32; 26] = [0; 26];

    for char_s in s.bytes() {
        count[(char_s - b'a') as usize] += 1
    }
    for char_t in t.bytes() {
        count[(char_t - b'a') as usize] -= 1
    }

    count.iter().all(|&x| x == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let s: String = String::from("anagram");
        let t: String = String::from("nagaram");
        assert_eq!(valid_anagram(s, t), true);
    }

    #[test]
    fn case_2() {
        let s: String = String::from("rat");
        let t: String = String::from("car");
        assert_eq!(valid_anagram(s, t), false);
    }
}
