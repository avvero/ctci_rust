use std::collections::HashMap;

struct Solution;

// https://leetcode.com/problems/two-sum
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, &e) in nums.iter().enumerate() {
            map.insert(e, i as i32);
        }
        for (i, &e) in nums.iter().enumerate() {
            let j = map.get(&(target - e));
            match j {
                Some(&it) if it != i as i32 => {
                    return vec![i as i32, it];
                }
                _ => continue,
            }
        }
        return nums;
    }
}

#[cfg(test)]
mod tests {
    use crate::solution1::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::two_sum(vec![2, 5, 5, 11], 10), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}