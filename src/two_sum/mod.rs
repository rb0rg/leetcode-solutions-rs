#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indices: Vec<i32> = vec![];
    for (i, n) in nums.iter().enumerate() {
        for next_i in i + 1..nums.len() {
            if let Some(next_n) = nums.get(next_i) {
                if n + next_n == target {
                    indices.push(i as i32);
                    indices.push(next_i as i32);
                    break;
                }
            }
        }
    }

    indices
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
