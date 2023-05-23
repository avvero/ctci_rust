struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, e1) in nums.iter().enumerate() {
            for (j, e2) in nums.iter().enumerate() {
                if i != j && e1 + e2 == target {
                    return vec![i as i32, j as i32]
                }
            }
        }
        return nums
    }
}

#[cfg(test)]
mod tests {
    use crate::solution1::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0,1]);
        assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1,2]);
        assert_eq!(Solution::two_sum(vec![3,3], 6), vec![0,1]);
    }
}