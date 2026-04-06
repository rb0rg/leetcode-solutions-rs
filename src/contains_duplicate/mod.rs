use std::collections::HashSet;

#[allow(dead_code)]
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();
    for num in nums.into_iter() {
        if !set.insert(num) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums: Vec<i32> = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    fn case_2() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(contains_duplicate(nums), false);
    }

    #[test]
    fn case_3() {
        let nums: Vec<i32> = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(contains_duplicate(nums), true);
    }
}
