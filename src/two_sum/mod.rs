use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let need = target - num;
        if let Some(&index) = num_map.get(&need) {
            return vec![index as i32, i as i32];
        }
        num_map.insert(num, i);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn case_2() {
        let nums: Vec<i32> = vec![3, 2, 4];
        let target: i32 = 6;
        assert_eq!(two_sum(nums, target), vec![1, 2]);
    }

    #[test]
    fn case_3() {
        let nums: Vec<i32> = vec![3, 3];
        let target: i32 = 6;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}
